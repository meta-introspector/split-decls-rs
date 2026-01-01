/* FP:mod.rs-0001 */ // Finding the dominators in a control-flow graph.
/* FP:mod.rs-0002 */ //
/* FP:mod.rs-0003 */ // Algorithm based on Loukas Georgiadis,
/* FP:mod.rs-0004 */ // "Linear-Time Algorithms for Dominators and Related Problems",
/* FP:mod.rs-0005 */ // <https://www.cs.princeton.edu/techreports/2005/737.pdf>
/* FP:mod.rs-0006 */ //
/* FP:mod.rs-0007 */ // Additionally useful is the original Lengauer-Tarjan paper on this subject,
/* FP:mod.rs-0008 */ // "A Fast Algorithm for Finding Dominators in a Flowgraph"
/* FP:mod.rs-0009 */ // Thomas Lengauer and Robert Endre Tarjan.
/* FP:mod.rs-0010 */ // <https://www.cs.princeton.edu/courses/archive/spr03/cs423/download/dominators.pdf>
/* FP:mod.rs-0011 */ 
/* FP:mod.rs-0012 */ use crate::rustc_index::{Idx, IndexSlice, IndexVec};
/* FP:mod.rs-0013 */ 
/* FP:mod.rs-0014 */ use super::ControlFlowGraph;
/* FP:mod.rs-0015 */ 
/* FP:mod.rs-0016 */ #[cfg(test)]
/* FP:mod.rs-0018 */ 
/* FP:mod.rs-0019 */ struct PreOrderFrame<Iter> {
/* FP:mod.rs-0020 */     pre_order_idx: PreorderIndex,
/* FP:mod.rs-0021 */     iter: Iter,
/* FP:mod.rs-0022 */ }
/* FP:mod.rs-0023 */ 
/* FP:mod.rs-0024 */ crate::rustc_index::newtype_index! {
/* FP:mod.rs-0025 */     #[orderable]
/* FP:mod.rs-0026 */     struct PreorderIndex {}
/* FP:mod.rs-0027 */ }
/* FP:mod.rs-0028 */ 
/* FP:mod.rs-0029 */ #[derive(Clone, Debug)]
/* FP:mod.rs-0030 */ pub struct Dominators<Node: Idx> {
/* FP:mod.rs-0031 */     kind: Kind<Node>,
/* FP:mod.rs-0032 */ }
/* FP:mod.rs-0033 */ 
/* FP:mod.rs-0034 */ #[derive(Clone, Debug)]
/* FP:mod.rs-0035 */ enum Kind<Node: Idx> {
/* FP:mod.rs-0036 */     /// A representation optimized for a small path graphs.
/* FP:mod.rs-0037 */     Path,
/* FP:mod.rs-0038 */     General(Inner<Node>),
/* FP:mod.rs-0039 */ }
/* FP:mod.rs-0040 */ 
/* FP:mod.rs-0041 */ pub fn dominators<G: ControlFlowGraph>(g: &G) -> Dominators<G::Node> {
/* FP:mod.rs-0042 */     // We often encounter MIR bodies with 1 or 2 basic blocks. Special case the dominators
/* FP:mod.rs-0043 */     // computation and representation for those cases.
/* FP:mod.rs-0044 */     if is_small_path_graph(g) {
/* FP:mod.rs-0045 */         Dominators { kind: Kind::Path }
/* FP:mod.rs-0046 */     } else {
/* FP:mod.rs-0047 */         Dominators { kind: Kind::General(dominators_impl(g)) }
/* FP:mod.rs-0048 */     }
/* FP:mod.rs-0049 */ }
/* FP:mod.rs-0050 */ 
/* FP:mod.rs-0051 */ fn is_small_path_graph<G: ControlFlowGraph>(g: &G) -> bool {
/* FP:mod.rs-0052 */     if g.start_node().index() != 0 {
/* FP:mod.rs-0053 */         return false;
/* FP:mod.rs-0054 */     }
/* FP:mod.rs-0055 */     if g.num_nodes() == 1 {
/* FP:mod.rs-0056 */         return true;
/* FP:mod.rs-0057 */     }
/* FP:mod.rs-0058 */     if g.num_nodes() == 2 {
/* FP:mod.rs-0059 */         return g.successors(g.start_node()).any(|n| n.index() == 1);
/* FP:mod.rs-0060 */     }
/* FP:mod.rs-0061 */     false
/* FP:mod.rs-0062 */ }
/* FP:mod.rs-0063 */ 
/* FP:mod.rs-0064 */ fn dominators_impl<G: ControlFlowGraph>(graph: &G) -> Inner<G::Node> {
/* FP:mod.rs-0065 */     // We allocate capacity for the full set of nodes, because most of the time
/* FP:mod.rs-0066 */     // most of the nodes *are* reachable.
/* FP:mod.rs-0067 */     let mut parent: IndexVec<PreorderIndex, PreorderIndex> =
/* FP:mod.rs-0068 */         IndexVec::with_capacity(graph.num_nodes());
/* FP:mod.rs-0069 */ 
/* FP:mod.rs-0070 */     let mut stack = vec![PreOrderFrame {
/* FP:mod.rs-0071 */         pre_order_idx: PreorderIndex::ZERO,
/* FP:mod.rs-0072 */         iter: graph.successors(graph.start_node()),
/* FP:mod.rs-0073 */     }];
/* FP:mod.rs-0074 */     let mut pre_order_to_real: IndexVec<PreorderIndex, G::Node> =
/* FP:mod.rs-0075 */         IndexVec::with_capacity(graph.num_nodes());
/* FP:mod.rs-0076 */     let mut real_to_pre_order: IndexVec<G::Node, Option<PreorderIndex>> =
/* FP:mod.rs-0077 */         IndexVec::from_elem_n(None, graph.num_nodes());
/* FP:mod.rs-0078 */     pre_order_to_real.push(graph.start_node());
/* FP:mod.rs-0079 */     parent.push(PreorderIndex::ZERO); // the parent of the root node is the root for now.
/* FP:mod.rs-0080 */     real_to_pre_order[graph.start_node()] = Some(PreorderIndex::ZERO);
/* FP:mod.rs-0081 */ 
/* FP:mod.rs-0082 */     // Traverse the graph, collecting a number of things:
/* FP:mod.rs-0083 */     //
/* FP:mod.rs-0084 */     // * Preorder mapping (to it, and back to the actual ordering)
/* FP:mod.rs-0085 */     // * Parents for each vertex in the preorder tree
/* FP:mod.rs-0086 */     //
/* FP:mod.rs-0087 */     // These are all done here rather than through one of the 'standard'
/* FP:mod.rs-0088 */     // graph traversals to help make this fast.
/* FP:mod.rs-0089 */     'recurse: while let Some(frame) = stack.last_mut() {
/* FP:mod.rs-0090 */         for successor in frame.iter.by_ref() {
/* FP:mod.rs-0091 */             if real_to_pre_order[successor].is_none() {
/* FP:mod.rs-0092 */                 let pre_order_idx = pre_order_to_real.push(successor);
/* FP:mod.rs-0093 */                 real_to_pre_order[successor] = Some(pre_order_idx);
/* FP:mod.rs-0094 */                 parent.push(frame.pre_order_idx);
/* FP:mod.rs-0095 */                 stack.push(PreOrderFrame { pre_order_idx, iter: graph.successors(successor) });
/* FP:mod.rs-0096 */ 
/* FP:mod.rs-0097 */                 continue 'recurse;
/* FP:mod.rs-0098 */             }
/* FP:mod.rs-0099 */         }
/* FP:mod.rs-0100 */ 
/* FP:mod.rs-0101 */         stack.pop();
/* FP:mod.rs-0102 */     }
/* FP:mod.rs-0103 */ 
/* FP:mod.rs-0104 */     let reachable_vertices = pre_order_to_real.len();
/* FP:mod.rs-0105 */ 
/* FP:mod.rs-0106 */     let mut idom = IndexVec::from_elem_n(PreorderIndex::ZERO, reachable_vertices);
/* FP:mod.rs-0107 */     let mut semi = IndexVec::from_fn_n(std::convert::identity, reachable_vertices);
/* FP:mod.rs-0108 */     let mut label = semi.clone();
/* FP:mod.rs-0109 */     let mut bucket = IndexVec::from_elem_n(vec![], reachable_vertices);
/* FP:mod.rs-0110 */     let mut lastlinked = None;
/* FP:mod.rs-0111 */ 
/* FP:mod.rs-0112 */     // We loop over vertices in reverse preorder. This implements the pseudocode
/* FP:mod.rs-0113 */     // of the simple Lengauer-Tarjan algorithm. A few key facts are noted here
/* FP:mod.rs-0114 */     // which are helpful for understanding the code (full proofs and such are
/* FP:mod.rs-0115 */     // found in various papers, including one cited at the top of this file).
/* FP:mod.rs-0116 */     //
/* FP:mod.rs-0117 */     // For each vertex w (which is not the root),
/* FP:mod.rs-0118 */     //  * semi[w] is a proper ancestor of the vertex w (i.e., semi[w] != w)
/* FP:mod.rs-0119 */     //  * idom[w] is an ancestor of semi[w] (i.e., idom[w] may equal semi[w])
/* FP:mod.rs-0120 */     //
/* FP:mod.rs-0121 */     // An immediate dominator of w (idom[w]) is a vertex v where v dominates w
/* FP:mod.rs-0122 */     // and every other dominator of w dominates v. (Every vertex except the root has
/* FP:mod.rs-0123 */     // a unique immediate dominator.)
/* FP:mod.rs-0124 */     //
/* FP:mod.rs-0125 */     // A semidominator for a given vertex w (semi[w]) is the vertex v with minimum
/* FP:mod.rs-0126 */     // preorder number such that there exists a path from v to w in which all elements (other than w) have
/* FP:mod.rs-0127 */     // preorder numbers greater than w (i.e., this path is not the tree path to
/* FP:mod.rs-0128 */     // w).
/* FP:mod.rs-0129 */     for w in (PreorderIndex::new(1)..PreorderIndex::new(reachable_vertices)).rev() {
/* FP:mod.rs-0130 */         // Optimization: process buckets just once, at the start of the
/* FP:mod.rs-0131 */         // iteration. Do not explicitly empty the bucket (even though it will
/* FP:mod.rs-0132 */         // not be used again), to save some instructions.
/* FP:mod.rs-0133 */         //
/* FP:mod.rs-0134 */         // The bucket here contains the vertices whose semidominator is the
/* FP:mod.rs-0135 */         // vertex w, which we are guaranteed to have found: all vertices who can
/* FP:mod.rs-0136 */         // be semidominated by w must have a preorder number exceeding w, so
/* FP:mod.rs-0137 */         // they have been placed in the bucket.
/* FP:mod.rs-0138 */         //
/* FP:mod.rs-0139 */         // We compute a partial set of immediate dominators here.
/* FP:mod.rs-0140 */         for &v in bucket[w].iter() {
/* FP:mod.rs-0141 */             // This uses the result of Lemma 5 from section 2 from the original
/* FP:mod.rs-0142 */             // 1979 paper, to compute either the immediate or relative dominator
/* FP:mod.rs-0143 */             // for a given vertex v.
/* FP:mod.rs-0144 */             //
/* FP:mod.rs-0145 */             // eval returns a vertex y, for which semi[y] is minimum among
/* FP:mod.rs-0146 */             // vertices semi[v] +> y *> v. Note that semi[v] = w as we're in the
/* FP:mod.rs-0147 */             // w bucket.
/* FP:mod.rs-0148 */             //
/* FP:mod.rs-0149 */             // Given such a vertex y, semi[y] <= semi[v] and idom[y] = idom[v].
/* FP:mod.rs-0150 */             // If semi[y] = semi[v], though, idom[v] = semi[v].
/* FP:mod.rs-0151 */             //
/* FP:mod.rs-0152 */             // Using this, we can either set idom[v] to be:
/* FP:mod.rs-0153 */             //  * semi[v] (i.e. w), if semi[y] is w
/* FP:mod.rs-0154 */             //  * idom[y], otherwise
/* FP:mod.rs-0155 */             //
/* FP:mod.rs-0156 */             // We don't directly set to idom[y] though as it's not necessarily
/* FP:mod.rs-0157 */             // known yet. The second preorder traversal will cleanup by updating
/* FP:mod.rs-0158 */             // the idom for any that were missed in this pass.
/* FP:mod.rs-0159 */             let y = eval(&mut parent, lastlinked, &semi, &mut label, v);
/* FP:mod.rs-0160 */             idom[v] = if semi[y] < w { y } else { w };
/* FP:mod.rs-0161 */         }
/* FP:mod.rs-0162 */ 
/* FP:mod.rs-0163 */         // This loop computes the semi[w] for w.
/* FP:mod.rs-0164 */         semi[w] = w;
/* FP:mod.rs-0165 */         for v in graph.predecessors(pre_order_to_real[w]) {
/* FP:mod.rs-0166 */             // TL;DR: Reachable vertices may have unreachable predecessors, so ignore any of them.
/* FP:mod.rs-0167 */             //
/* FP:mod.rs-0168 */             // Ignore blocks which are not connected to the entry block.
/* FP:mod.rs-0169 */             //
/* FP:mod.rs-0170 */             // The algorithm that was used to traverse the graph and build the
/* FP:mod.rs-0171 */             // `pre_order_to_real` and `real_to_pre_order` vectors does so by
/* FP:mod.rs-0172 */             // starting from the entry block and following the successors.
/* FP:mod.rs-0173 */             // Therefore, any blocks not reachable from the entry block will be
/* FP:mod.rs-0174 */             // set to `None` in the `pre_order_to_real` vector.
/* FP:mod.rs-0175 */             //
/* FP:mod.rs-0176 */             // For example, in this graph, A and B should be skipped:
/* FP:mod.rs-0177 */             //
/* FP:mod.rs-0178 */             //           ┌─────┐
/* FP:mod.rs-0179 */             //           │     │
/* FP:mod.rs-0180 */             //           └──┬──┘
/* FP:mod.rs-0181 */             //              │
/* FP:mod.rs-0182 */             //           ┌──▼──┐              ┌─────┐
/* FP:mod.rs-0183 */             //           │     │              │  A  │
/* FP:mod.rs-0184 */             //           └──┬──┘              └──┬──┘
/* FP:mod.rs-0185 */             //              │                    │
/* FP:mod.rs-0186 */             //      ┌───────┴───────┐            │
/* FP:mod.rs-0187 */             //      │               │            │
/* FP:mod.rs-0188 */             //   ┌──▼──┐         ┌──▼──┐      ┌──▼──┐
/* FP:mod.rs-0189 */             //   │     │         │     │      │  B  │
/* FP:mod.rs-0190 */             //   └──┬──┘         └──┬──┘      └──┬──┘
/* FP:mod.rs-0191 */             //      │               └──────┬─────┘
/* FP:mod.rs-0192 */             //   ┌──▼──┐                   │
/* FP:mod.rs-0193 */             //   │     │                   │
/* FP:mod.rs-0194 */             //   └──┬──┘                ┌──▼──┐
/* FP:mod.rs-0195 */             //      │                   │     │
/* FP:mod.rs-0196 */             //      │                   └─────┘
/* FP:mod.rs-0197 */             //   ┌──▼──┐
/* FP:mod.rs-0198 */             //   │     │
/* FP:mod.rs-0199 */             //   └──┬──┘
/* FP:mod.rs-0200 */             //      │
/* FP:mod.rs-0201 */             //   ┌──▼──┐
/* FP:mod.rs-0202 */             //   │     │
/* FP:mod.rs-0203 */             //   └─────┘
/* FP:mod.rs-0204 */             //
/* FP:mod.rs-0205 */             // ...this may be the case if a MirPass modifies the CFG to remove
/* FP:mod.rs-0206 */             // or rearrange certain blocks/edges.
/* FP:mod.rs-0207 */             let Some(v) = real_to_pre_order[v] else { continue };
/* FP:mod.rs-0208 */ 
/* FP:mod.rs-0209 */             // eval returns a vertex x from which semi[x] is minimum among
/* FP:mod.rs-0210 */             // vertices semi[v] +> x *> v.
/* FP:mod.rs-0211 */             //
/* FP:mod.rs-0212 */             // From Lemma 4 from section 2, we know that the semidominator of a
/* FP:mod.rs-0213 */             // vertex w is the minimum (by preorder number) vertex of the
/* FP:mod.rs-0214 */             // following:
/* FP:mod.rs-0215 */             //
/* FP:mod.rs-0216 */             //  * direct predecessors of w with preorder number less than w
/* FP:mod.rs-0217 */             //  * semidominators of u such that u > w and there exists (v, w)
/* FP:mod.rs-0218 */             //    such that u *> v
/* FP:mod.rs-0219 */             //
/* FP:mod.rs-0220 */             // This loop therefore identifies such a minima. Note that any
/* FP:mod.rs-0221 */             // semidominator path to w must have all but the first vertex go
/* FP:mod.rs-0222 */             // through vertices numbered greater than w, so the reverse preorder
/* FP:mod.rs-0223 */             // traversal we are using guarantees that all of the information we
/* FP:mod.rs-0224 */             // might need is available at this point.
/* FP:mod.rs-0225 */             //
/* FP:mod.rs-0226 */             // The eval call will give us semi[x], which is either:
/* FP:mod.rs-0227 */             //
/* FP:mod.rs-0228 */             //  * v itself, if v has not yet been processed
/* FP:mod.rs-0229 */             //  * A possible 'best' semidominator for w.
/* FP:mod.rs-0230 */             let x = eval(&mut parent, lastlinked, &semi, &mut label, v);
/* FP:mod.rs-0231 */             semi[w] = std::cmp::min(semi[w], semi[x]);
/* FP:mod.rs-0232 */         }
/* FP:mod.rs-0233 */         // semi[w] is now semidominator(w) and won't change any more.
/* FP:mod.rs-0234 */ 
/* FP:mod.rs-0235 */         // Optimization: Do not insert into buckets if parent[w] = semi[w], as
/* FP:mod.rs-0236 */         // we then immediately know the idom.
/* FP:mod.rs-0237 */         //
/* FP:mod.rs-0238 */         // If we don't yet know the idom directly, then push this vertex into
/* FP:mod.rs-0239 */         // our semidominator's bucket, where it will get processed at a later
/* FP:mod.rs-0240 */         // stage to compute its immediate dominator.
/* FP:mod.rs-0241 */         let z = parent[w];
/* FP:mod.rs-0242 */         if z != semi[w] {
/* FP:mod.rs-0243 */             bucket[semi[w]].push(w);
/* FP:mod.rs-0244 */         } else {
/* FP:mod.rs-0245 */             idom[w] = z;
/* FP:mod.rs-0246 */         }
/* FP:mod.rs-0247 */ 
/* FP:mod.rs-0248 */         // Optimization: We share the parent array between processed and not
/* FP:mod.rs-0249 */         // processed elements; lastlinked represents the divider.
/* FP:mod.rs-0250 */         lastlinked = Some(w);
/* FP:mod.rs-0251 */     }
/* FP:mod.rs-0252 */ 
/* FP:mod.rs-0253 */     // Finalize the idoms for any that were not fully settable during initial
/* FP:mod.rs-0254 */     // traversal.
/* FP:mod.rs-0255 */     //
/* FP:mod.rs-0256 */     // If idom[w] != semi[w] then we know that we've stored vertex y from above
/* FP:mod.rs-0257 */     // into idom[w]. It is known to be our 'relative dominator', which means
/* FP:mod.rs-0258 */     // that it's one of w's ancestors and has the same immediate dominator as w,
/* FP:mod.rs-0259 */     // so use that idom.
/* FP:mod.rs-0260 */     for w in PreorderIndex::new(1)..PreorderIndex::new(reachable_vertices) {
/* FP:mod.rs-0261 */         if idom[w] != semi[w] {
/* FP:mod.rs-0262 */             idom[w] = idom[idom[w]];
/* FP:mod.rs-0263 */         }
/* FP:mod.rs-0264 */     }
/* FP:mod.rs-0265 */ 
/* FP:mod.rs-0266 */     let mut immediate_dominators = IndexVec::from_elem_n(None, graph.num_nodes());
/* FP:mod.rs-0267 */     for (idx, node) in pre_order_to_real.iter_enumerated() {
/* FP:mod.rs-0268 */         immediate_dominators[*node] = Some(pre_order_to_real[idom[idx]]);
/* FP:mod.rs-0269 */     }
/* FP:mod.rs-0270 */ 
/* FP:mod.rs-0271 */     let start_node = graph.start_node();
/* FP:mod.rs-0272 */     immediate_dominators[start_node] = None;
/* FP:mod.rs-0273 */ 
/* FP:mod.rs-0274 */     let time = compute_access_time(start_node, &immediate_dominators);
/* FP:mod.rs-0275 */ 
/* FP:mod.rs-0276 */     Inner { immediate_dominators, time }
/* FP:mod.rs-0277 */ }
/* FP:mod.rs-0278 */ 
/* FP:mod.rs-0279 */ /// Evaluate the link-eval virtual forest, providing the currently minimum semi
/* FP:mod.rs-0280 */ /// value for the passed `node` (which may be itself).
/* FP:mod.rs-0281 */ ///
/* FP:mod.rs-0282 */ /// This maintains that for every vertex v, `label[v]` is such that:
/* FP:mod.rs-0283 */ ///
/* FP:mod.rs-0284 */ /// ```text
/* FP:mod.rs-0285 */ /// semi[eval(v)] = min { semi[label[u]] | root_in_forest(v) +> u *> v }
/* FP:mod.rs-0286 */ /// ```
/* FP:mod.rs-0287 */ ///
/* FP:mod.rs-0288 */ /// where `+>` is a proper ancestor and `*>` is just an ancestor.
/* FP:mod.rs-0289 */ #[inline]
/* FP:mod.rs-0290 */ fn eval(
/* FP:mod.rs-0291 */     ancestor: &mut IndexSlice<PreorderIndex, PreorderIndex>,
/* FP:mod.rs-0292 */     lastlinked: Option<PreorderIndex>,
/* FP:mod.rs-0293 */     semi: &IndexSlice<PreorderIndex, PreorderIndex>,
/* FP:mod.rs-0294 */     label: &mut IndexSlice<PreorderIndex, PreorderIndex>,
/* FP:mod.rs-0295 */     node: PreorderIndex,
/* FP:mod.rs-0296 */ ) -> PreorderIndex {
/* FP:mod.rs-0297 */     if is_processed(node, lastlinked) {
/* FP:mod.rs-0298 */         compress(ancestor, lastlinked, semi, label, node);
/* FP:mod.rs-0299 */         label[node]
/* FP:mod.rs-0300 */     } else {
/* FP:mod.rs-0301 */         node
/* FP:mod.rs-0302 */     }
/* FP:mod.rs-0303 */ }
/* FP:mod.rs-0304 */ 
/* FP:mod.rs-0305 */ #[inline]
/* FP:mod.rs-0306 */ fn is_processed(v: PreorderIndex, lastlinked: Option<PreorderIndex>) -> bool {
/* FP:mod.rs-0307 */     if let Some(ll) = lastlinked { v >= ll } else { false }
/* FP:mod.rs-0308 */ }
/* FP:mod.rs-0309 */ 
/* FP:mod.rs-0310 */ #[inline]
/* FP:mod.rs-0311 */ fn compress(
/* FP:mod.rs-0312 */     ancestor: &mut IndexSlice<PreorderIndex, PreorderIndex>,
/* FP:mod.rs-0313 */     lastlinked: Option<PreorderIndex>,
/* FP:mod.rs-0314 */     semi: &IndexSlice<PreorderIndex, PreorderIndex>,
/* FP:mod.rs-0315 */     label: &mut IndexSlice<PreorderIndex, PreorderIndex>,
/* FP:mod.rs-0316 */     v: PreorderIndex,
/* FP:mod.rs-0317 */ ) {
/* FP:mod.rs-0318 */     assert!(is_processed(v, lastlinked));
/* FP:mod.rs-0319 */     // Compute the processed list of ancestors
/* FP:mod.rs-0320 */     //
/* FP:mod.rs-0321 */     // We use a heap stack here to avoid recursing too deeply, exhausting the
/* FP:mod.rs-0322 */     // stack space.
/* FP:mod.rs-0323 */     let mut stack: smallvec::SmallVec<[_; 8]> = smallvec::smallvec![v];
/* FP:mod.rs-0324 */     let mut u = ancestor[v];
/* FP:mod.rs-0325 */     while is_processed(u, lastlinked) {
/* FP:mod.rs-0326 */         stack.push(u);
/* FP:mod.rs-0327 */         u = ancestor[u];
/* FP:mod.rs-0328 */     }
/* FP:mod.rs-0329 */ 
/* FP:mod.rs-0330 */     // Then in reverse order, popping the stack
/* FP:mod.rs-0331 */     for &[v, u] in stack.array_windows().rev() {
/* FP:mod.rs-0332 */         if semi[label[u]] < semi[label[v]] {
/* FP:mod.rs-0333 */             label[v] = label[u];
/* FP:mod.rs-0334 */         }
/* FP:mod.rs-0335 */         ancestor[v] = ancestor[u];
/* FP:mod.rs-0336 */     }
/* FP:mod.rs-0337 */ }
/* FP:mod.rs-0338 */ 
/* FP:mod.rs-0339 */ /// Tracks the list of dominators for each node.
/* FP:mod.rs-0340 */ #[derive(Clone, Debug)]
/* FP:mod.rs-0341 */ struct Inner<N: Idx> {
/* FP:mod.rs-0342 */     // Even though we track only the immediate dominator of each node, it's
/* FP:mod.rs-0343 */     // possible to get its full list of dominators by looking up the dominator
/* FP:mod.rs-0344 */     // of each dominator.
/* FP:mod.rs-0345 */     immediate_dominators: IndexVec<N, Option<N>>,
/* FP:mod.rs-0346 */     time: IndexVec<N, Time>,
/* FP:mod.rs-0347 */ }
/* FP:mod.rs-0348 */ 
/* FP:mod.rs-0349 */ impl<Node: Idx> Dominators<Node> {
/* FP:mod.rs-0350 */     /// Returns true if node is reachable from the start node.
/* FP:mod.rs-0351 */     pub fn is_reachable(&self, node: Node) -> bool {
/* FP:mod.rs-0352 */         match &self.kind {
/* FP:mod.rs-0353 */             Kind::Path => true,
/* FP:mod.rs-0354 */             Kind::General(g) => g.time[node].start != 0,
/* FP:mod.rs-0355 */         }
/* FP:mod.rs-0356 */     }
/* FP:mod.rs-0357 */ 
/* FP:mod.rs-0358 */     /// Returns the immediate dominator of node, if any.
/* FP:mod.rs-0359 */     pub fn immediate_dominator(&self, node: Node) -> Option<Node> {
/* FP:mod.rs-0360 */         match &self.kind {
/* FP:mod.rs-0361 */             Kind::Path => {
/* FP:mod.rs-0362 */                 if 0 < node.index() {
/* FP:mod.rs-0363 */                     Some(Node::new(node.index() - 1))
/* FP:mod.rs-0364 */                 } else {
/* FP:mod.rs-0365 */                     None
/* FP:mod.rs-0366 */                 }
/* FP:mod.rs-0367 */             }
/* FP:mod.rs-0368 */             Kind::General(g) => g.immediate_dominators[node],
/* FP:mod.rs-0369 */         }
/* FP:mod.rs-0370 */     }
/* FP:mod.rs-0371 */ 
/* FP:mod.rs-0372 */     /// Returns true if `a` dominates `b`.
/* FP:mod.rs-0373 */     ///
/* FP:mod.rs-0374 */     /// # Panics
/* FP:mod.rs-0375 */     ///
/* FP:mod.rs-0376 */     /// Panics if `b` is unreachable.
/* FP:mod.rs-0377 */     #[inline]
/* FP:mod.rs-0378 */     pub fn dominates(&self, a: Node, b: Node) -> bool {
/* FP:mod.rs-0379 */         match &self.kind {
/* FP:mod.rs-0380 */             Kind::Path => a.index() <= b.index(),
/* FP:mod.rs-0381 */             Kind::General(g) => {
/* FP:mod.rs-0382 */                 let a = g.time[a];
/* FP:mod.rs-0383 */                 let b = g.time[b];
/* FP:mod.rs-0384 */                 assert!(b.start != 0, "node {b:?} is not reachable");
/* FP:mod.rs-0385 */                 a.start <= b.start && b.finish <= a.finish
/* FP:mod.rs-0386 */             }
/* FP:mod.rs-0387 */         }
/* FP:mod.rs-0388 */     }
/* FP:mod.rs-0389 */ }
/* FP:mod.rs-0390 */ 
/* FP:mod.rs-0391 */ /// Describes the number of vertices discovered at the time when processing of a particular vertex
/* FP:mod.rs-0392 */ /// started and when it finished. Both values are zero for unreachable vertices.
/* FP:mod.rs-0393 */ #[derive(Copy, Clone, Default, Debug)]
/* FP:mod.rs-0394 */ struct Time {
/* FP:mod.rs-0395 */     start: u32,
/* FP:mod.rs-0396 */     finish: u32,
/* FP:mod.rs-0397 */ }
/* FP:mod.rs-0398 */ 
/* FP:mod.rs-0399 */ fn compute_access_time<N: Idx>(
/* FP:mod.rs-0400 */     start_node: N,
/* FP:mod.rs-0401 */     immediate_dominators: &IndexSlice<N, Option<N>>,
/* FP:mod.rs-0402 */ ) -> IndexVec<N, Time> {
/* FP:mod.rs-0403 */     // Transpose the dominator tree edges, so that child nodes of vertex v are stored in
/* FP:mod.rs-0404 */     // node[edges[v].start..edges[v].end].
/* FP:mod.rs-0405 */     let mut edges: IndexVec<N, std::ops::Range<u32>> =
/* FP:mod.rs-0406 */         IndexVec::from_elem(0..0, immediate_dominators);
/* FP:mod.rs-0407 */     for &idom in immediate_dominators.iter() {
/* FP:mod.rs-0408 */         if let Some(idom) = idom {
/* FP:mod.rs-0409 */             edges[idom].end += 1;
/* FP:mod.rs-0410 */         }
/* FP:mod.rs-0411 */     }
/* FP:mod.rs-0412 */     let mut m = 0;
/* FP:mod.rs-0413 */     for e in edges.iter_mut() {
/* FP:mod.rs-0414 */         m += e.end;
/* FP:mod.rs-0415 */         e.start = m;
/* FP:mod.rs-0416 */         e.end = m;
/* FP:mod.rs-0417 */     }
/* FP:mod.rs-0418 */     let mut node = IndexVec::from_elem_n(Idx::new(0), m.try_into().unwrap());
/* FP:mod.rs-0419 */     for (i, &idom) in immediate_dominators.iter_enumerated() {
/* FP:mod.rs-0420 */         if let Some(idom) = idom {
/* FP:mod.rs-0421 */             edges[idom].start -= 1;
/* FP:mod.rs-0422 */             node[edges[idom].start] = i;
/* FP:mod.rs-0423 */         }
/* FP:mod.rs-0424 */     }
/* FP:mod.rs-0425 */ 
/* FP:mod.rs-0426 */     // Perform a depth-first search of the dominator tree. Record the number of vertices discovered
/* FP:mod.rs-0427 */     // when vertex v is discovered first as time[v].start, and when its processing is finished as
/* FP:mod.rs-0428 */     // time[v].finish.
/* FP:mod.rs-0429 */     let mut time: IndexVec<N, Time> = IndexVec::from_elem(Time::default(), immediate_dominators);
/* FP:mod.rs-0430 */     let mut stack = Vec::new();
/* FP:mod.rs-0431 */ 
/* FP:mod.rs-0432 */     let mut discovered = 1;
/* FP:mod.rs-0433 */     stack.push(start_node);
/* FP:mod.rs-0434 */     time[start_node].start = discovered;
/* FP:mod.rs-0435 */ 
/* FP:mod.rs-0436 */     while let Some(&i) = stack.last() {
/* FP:mod.rs-0437 */         let e = &mut edges[i];
/* FP:mod.rs-0438 */         if e.start == e.end {
/* FP:mod.rs-0439 */             // Finish processing vertex i.
/* FP:mod.rs-0440 */             time[i].finish = discovered;
/* FP:mod.rs-0441 */             stack.pop();
/* FP:mod.rs-0442 */         } else {
/* FP:mod.rs-0443 */             let j = node[e.start];
/* FP:mod.rs-0444 */             e.start += 1;
/* FP:mod.rs-0445 */             // Start processing vertex j.
/* FP:mod.rs-0446 */             discovered += 1;
/* FP:mod.rs-0447 */             time[j].start = discovered;
/* FP:mod.rs-0448 */             stack.push(j);
/* FP:mod.rs-0449 */         }
/* FP:mod.rs-0450 */     }
/* FP:mod.rs-0451 */ 
/* FP:mod.rs-0452 */     time
/* FP:mod.rs-0453 */ }