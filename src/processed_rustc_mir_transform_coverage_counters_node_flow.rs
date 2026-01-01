/* FP:node_flow.rs-0001 */ // For each node in a control-flow graph, determines whether that node should
/* FP:node_flow.rs-0002 */ // have a physical counter, or a counter expression that is derived from the
/* FP:node_flow.rs-0003 */ // physical counters of other nodes.
/* FP:node_flow.rs-0004 */ //
/* FP:node_flow.rs-0005 */ // Based on the algorithm given in
/* FP:node_flow.rs-0006 */ // "Optimal measurement points for program frequency counts"
/* FP:node_flow.rs-0007 */ // (Knuth & Stevenson, 1973).
/* FP:node_flow.rs-0008 */ 
/* FP:node_flow.rs-0009 */ use crate::rustc_data_structures::graph;
/* FP:node_flow.rs-0010 */ use crate::rustc_data_structures::union_find::UnionFind;
/* FP:node_flow.rs-0011 */ use crate::rustc_index::bit_set::DenseBitSet;
/* FP:node_flow.rs-0012 */ use crate::rustc_index::{Idx, IndexSlice, IndexVec};
/* FP:node_flow.rs-0013 */ pub(crate) use crate::rustc_complete::mir::coverage::NodeFlowData;
/* FP:node_flow.rs-0014 */ use crate::rustc_complete::mir::coverage::Op;
/* FP:node_flow.rs-0015 */ 
/* FP:node_flow.rs-0016 */ #[cfg(test)]
/* FP:node_flow.rs-0018 */ 
/* FP:node_flow.rs-0019 */ /// Creates a "merged" view of an underlying graph.
/* FP:node_flow.rs-0020 */ ///
/* FP:node_flow.rs-0021 */ /// The given graph is assumed to have [“balanced flow”](balanced-flow),
/* FP:node_flow.rs-0022 */ /// though it does not necessarily have to be a `BalancedFlowGraph`.
/* FP:node_flow.rs-0023 */ ///
/* FP:node_flow.rs-0024 */ /// [balanced-flow]: `crate::coverage::counters::balanced_flow::BalancedFlowGraph`.
/* FP:node_flow.rs-0025 */ pub(crate) fn node_flow_data_for_balanced_graph<G>(graph: G) -> NodeFlowData<G::Node>
/* FP:node_flow.rs-0026 */ where
/* FP:node_flow.rs-0027 */     G: graph::Successors,
/* FP:node_flow.rs-0028 */ {
/* FP:node_flow.rs-0029 */     let mut supernodes = UnionFind::<G::Node>::new(graph.num_nodes());
/* FP:node_flow.rs-0030 */ 
/* FP:node_flow.rs-0031 */     // For each node, merge its successors into a single supernode, and
/* FP:node_flow.rs-0032 */     // arbitrarily choose one of those successors to represent all of them.
/* FP:node_flow.rs-0033 */     let successors = graph
/* FP:node_flow.rs-0034 */         .iter_nodes()
/* FP:node_flow.rs-0035 */         .map(|node| {
/* FP:node_flow.rs-0036 */             graph
/* FP:node_flow.rs-0037 */                 .successors(node)
/* FP:node_flow.rs-0038 */                 .reduce(|a, b| supernodes.unify(a, b))
/* FP:node_flow.rs-0039 */                 .expect("each node in a balanced graph must have at least one out-edge")
/* FP:node_flow.rs-0040 */         })
/* FP:node_flow.rs-0041 */         .collect::<IndexVec<G::Node, G::Node>>();
/* FP:node_flow.rs-0042 */ 
/* FP:node_flow.rs-0043 */     // Now that unification is complete, take a snapshot of the supernode forest,
/* FP:node_flow.rs-0044 */     // and resolve each arbitrarily-chosen successor to its canonical root.
/* FP:node_flow.rs-0045 */     // (This avoids having to explicitly resolve them later.)
/* FP:node_flow.rs-0046 */     let supernodes = supernodes.snapshot();
/* FP:node_flow.rs-0047 */     let succ_supernodes = successors.into_iter().map(|succ| supernodes[succ]).collect();
/* FP:node_flow.rs-0048 */ 
/* FP:node_flow.rs-0049 */     NodeFlowData { supernodes, succ_supernodes }
/* FP:node_flow.rs-0050 */ }
/* FP:node_flow.rs-0051 */ 
/* FP:node_flow.rs-0052 */ /// Uses the graph information in `node_flow_data`, together with a given
/* FP:node_flow.rs-0053 */ /// permutation of all nodes in the graph, to create physical counters and
/* FP:node_flow.rs-0054 */ /// counter expressions for each node in the underlying graph.
/* FP:node_flow.rs-0055 */ ///
/* FP:node_flow.rs-0056 */ /// The given list must contain exactly one copy of each node in the
/* FP:node_flow.rs-0057 */ /// underlying balanced-flow graph. The order of nodes is used as a hint to
/* FP:node_flow.rs-0058 */ /// influence counter allocation:
/* FP:node_flow.rs-0059 */ /// - Earlier nodes are more likely to receive counter expressions.
/* FP:node_flow.rs-0060 */ /// - Later nodes are more likely to receive physical counters.
/* FP:node_flow.rs-0061 */ pub(crate) fn make_node_counters<Node: Idx>(
/* FP:node_flow.rs-0062 */     node_flow_data: &NodeFlowData<Node>,
/* FP:node_flow.rs-0063 */     priority_list: &[Node],
/* FP:node_flow.rs-0064 */ ) -> NodeCounters<Node> {
/* FP:node_flow.rs-0065 */     let mut builder = SpantreeBuilder::new(node_flow_data);
/* FP:node_flow.rs-0066 */ 
/* FP:node_flow.rs-0067 */     for &node in priority_list {
/* FP:node_flow.rs-0068 */         builder.visit_node(node);
/* FP:node_flow.rs-0069 */     }
/* FP:node_flow.rs-0070 */ 
/* FP:node_flow.rs-0071 */     NodeCounters { counter_terms: builder.finish() }
/* FP:node_flow.rs-0072 */ }
/* FP:node_flow.rs-0073 */ 
/* FP:node_flow.rs-0074 */ /// End result of allocating physical counters and counter expressions for the
/* FP:node_flow.rs-0075 */ /// nodes of a graph.
/* FP:node_flow.rs-0076 */ #[derive(Debug)]
/* FP:node_flow.rs-0077 */ pub(crate) struct NodeCounters<Node: Idx> {
/* FP:node_flow.rs-0078 */     /// For the given node, returns the finished list of terms that represent
/* FP:node_flow.rs-0079 */     /// its physical counter or counter expression. Always non-empty.
/* FP:node_flow.rs-0080 */     ///
/* FP:node_flow.rs-0081 */     /// If a node was given a physical counter, the term list will contain
/* FP:node_flow.rs-0082 */     /// that counter as its sole element.
/* FP:node_flow.rs-0083 */     pub(crate) counter_terms: IndexVec<Node, Vec<CounterTerm<Node>>>,
/* FP:node_flow.rs-0084 */ }
/* FP:node_flow.rs-0085 */ 
/* FP:node_flow.rs-0086 */ #[derive(Debug)]
/* FP:node_flow.rs-0087 */ struct SpantreeEdge<Node> {
/* FP:node_flow.rs-0088 */     /// If true, this edge in the spantree has been reversed an odd number of
/* FP:node_flow.rs-0089 */     /// times, so all physical counters added to its node's counter expression
/* FP:node_flow.rs-0090 */     /// need to be negated.
/* FP:node_flow.rs-0091 */     is_reversed: bool,
/* FP:node_flow.rs-0092 */     /// Each spantree edge is "claimed" by the (regular) node that caused it to
/* FP:node_flow.rs-0093 */     /// be created. When a node with a physical counter traverses this edge,
/* FP:node_flow.rs-0094 */     /// that counter is added to the claiming node's counter expression.
/* FP:node_flow.rs-0095 */     claiming_node: Node,
/* FP:node_flow.rs-0096 */     /// Supernode at the other end of this spantree edge. Transitively points
/* FP:node_flow.rs-0097 */     /// to the "root" of this supernode's spantree component.
/* FP:node_flow.rs-0098 */     span_parent: Node,
/* FP:node_flow.rs-0099 */ }
/* FP:node_flow.rs-0100 */ 
/* FP:node_flow.rs-0101 */ /// Part of a node's counter expression, which is a sum of counter terms.
/* FP:node_flow.rs-0102 */ #[derive(Debug)]
/* FP:node_flow.rs-0103 */ pub(crate) struct CounterTerm<Node> {
/* FP:node_flow.rs-0104 */     /// Whether to add or subtract the value of the node's physical counter.
/* FP:node_flow.rs-0105 */     pub(crate) op: Op,
/* FP:node_flow.rs-0106 */     /// The node whose physical counter is represented by this term.
/* FP:node_flow.rs-0107 */     pub(crate) node: Node,
/* FP:node_flow.rs-0108 */ }
/* FP:node_flow.rs-0109 */ 
/* FP:node_flow.rs-0110 */ #[derive(Debug)]
/* FP:node_flow.rs-0111 */ struct SpantreeBuilder<'a, Node: Idx> {
/* FP:node_flow.rs-0112 */     supernodes: &'a IndexSlice<Node, Node>,
/* FP:node_flow.rs-0113 */     succ_supernodes: &'a IndexSlice<Node, Node>,
/* FP:node_flow.rs-0114 */ 
/* FP:node_flow.rs-0115 */     is_unvisited: DenseBitSet<Node>,
/* FP:node_flow.rs-0116 */     /// Links supernodes to each other, gradually forming a spanning tree of
/* FP:node_flow.rs-0117 */     /// the merged-flow graph.
/* FP:node_flow.rs-0118 */     ///
/* FP:node_flow.rs-0119 */     /// A supernode without a span edge is the root of its component of the
/* FP:node_flow.rs-0120 */     /// spantree. Nodes that aren't supernodes cannot have a spantree edge.
/* FP:node_flow.rs-0121 */     span_edges: IndexVec<Node, Option<SpantreeEdge<Node>>>,
/* FP:node_flow.rs-0122 */     /// Shared path buffer recycled by all calls to `yank_to_spantree_root`.
/* FP:node_flow.rs-0123 */     yank_buffer: Vec<Node>,
/* FP:node_flow.rs-0124 */     /// An in-progress counter expression for each node. Each expression is
/* FP:node_flow.rs-0125 */     /// initially empty, and will be filled in as relevant nodes are visited.
/* FP:node_flow.rs-0126 */     counter_terms: IndexVec<Node, Vec<CounterTerm<Node>>>,
/* FP:node_flow.rs-0127 */ }
/* FP:node_flow.rs-0128 */ 
/* FP:node_flow.rs-0129 */ impl<'a, Node: Idx> SpantreeBuilder<'a, Node> {
/* FP:node_flow.rs-0130 */     fn new(node_flow_data: &'a NodeFlowData<Node>) -> Self {
/* FP:node_flow.rs-0131 */         let NodeFlowData { supernodes, succ_supernodes } = node_flow_data;
/* FP:node_flow.rs-0132 */         let num_nodes = supernodes.len();
/* FP:node_flow.rs-0133 */         Self {
/* FP:node_flow.rs-0134 */             supernodes,
/* FP:node_flow.rs-0135 */             succ_supernodes,
/* FP:node_flow.rs-0136 */             is_unvisited: DenseBitSet::new_filled(num_nodes),
/* FP:node_flow.rs-0137 */             span_edges: IndexVec::from_fn_n(|_| None, num_nodes),
/* FP:node_flow.rs-0138 */             yank_buffer: vec![],
/* FP:node_flow.rs-0139 */             counter_terms: IndexVec::from_fn_n(|_| vec![], num_nodes),
/* FP:node_flow.rs-0140 */         }
/* FP:node_flow.rs-0141 */     }
/* FP:node_flow.rs-0142 */ 
/* FP:node_flow.rs-0143 */     fn is_supernode(&self, node: Node) -> bool {
/* FP:node_flow.rs-0144 */         self.supernodes[node] == node
/* FP:node_flow.rs-0145 */     }
/* FP:node_flow.rs-0146 */ 
/* FP:node_flow.rs-0147 */     /// Given a supernode, finds the supernode that is the "root" of its
/* FP:node_flow.rs-0148 */     /// spantree component. Two nodes that have the same spantree root are
/* FP:node_flow.rs-0149 */     /// connected in the spantree.
/* FP:node_flow.rs-0150 */     fn spantree_root(&self, this: Node) -> Node {
/* FP:node_flow.rs-0151 */         debug_assert!(self.is_supernode(this));
/* FP:node_flow.rs-0152 */ 
/* FP:node_flow.rs-0153 */         match self.span_edges[this] {
/* FP:node_flow.rs-0154 */             None => this,
/* FP:node_flow.rs-0155 */             Some(SpantreeEdge { span_parent, .. }) => self.spantree_root(span_parent),
/* FP:node_flow.rs-0156 */         }
/* FP:node_flow.rs-0157 */     }
/* FP:node_flow.rs-0158 */ 
/* FP:node_flow.rs-0159 */     /// Rotates edges in the spantree so that `this` is the root of its
/* FP:node_flow.rs-0160 */     /// spantree component.
/* FP:node_flow.rs-0161 */     fn yank_to_spantree_root(&mut self, this: Node) {
/* FP:node_flow.rs-0162 */         debug_assert!(self.is_supernode(this));
/* FP:node_flow.rs-0163 */ 
/* FP:node_flow.rs-0164 */         // The rotation is done iteratively, by first traversing from `this` to
/* FP:node_flow.rs-0165 */         // its root and storing the path in a buffer, and then traversing the
/* FP:node_flow.rs-0166 */         // path buffer backwards to reverse all the edges.
/* FP:node_flow.rs-0167 */ 
/* FP:node_flow.rs-0168 */         // Recycle the same path buffer for all calls to this method.
/* FP:node_flow.rs-0169 */         let path_buf = &mut self.yank_buffer;
/* FP:node_flow.rs-0170 */         path_buf.clear();
/* FP:node_flow.rs-0171 */         path_buf.push(this);
/* FP:node_flow.rs-0172 */ 
/* FP:node_flow.rs-0173 */         // Traverse the spantree until we reach a supernode that has no
/* FP:node_flow.rs-0174 */         // span-parent, which must be the root.
/* FP:node_flow.rs-0175 */         let mut curr = this;
/* FP:node_flow.rs-0176 */         while let &Some(SpantreeEdge { span_parent, .. }) = &self.span_edges[curr] {
/* FP:node_flow.rs-0177 */             path_buf.push(span_parent);
/* FP:node_flow.rs-0178 */             curr = span_parent;
/* FP:node_flow.rs-0179 */         }
/* FP:node_flow.rs-0180 */ 
/* FP:node_flow.rs-0181 */         // For each spantree edge `a -> b` in the path that was just traversed,
/* FP:node_flow.rs-0182 */         // reverse it to become `a <- b`, while preserving `claiming_node`.
/* FP:node_flow.rs-0183 */         for &[a, b] in path_buf.array_windows::<2>().rev() {
/* FP:node_flow.rs-0184 */             let SpantreeEdge { is_reversed, claiming_node, span_parent } = self.span_edges[a]
/* FP:node_flow.rs-0185 */                 .take()
/* FP:node_flow.rs-0186 */                 .expect("all nodes in the path (except the last) have a `span_parent`");
/* FP:node_flow.rs-0187 */             debug_assert_eq!(span_parent, b);
/* FP:node_flow.rs-0188 */             debug_assert!(self.span_edges[b].is_none());
/* FP:node_flow.rs-0189 */             self.span_edges[b] =
/* FP:node_flow.rs-0190 */                 Some(SpantreeEdge { is_reversed: !is_reversed, claiming_node, span_parent: a });
/* FP:node_flow.rs-0191 */         }
/* FP:node_flow.rs-0192 */ 
/* FP:node_flow.rs-0193 */         // The result of the rotation is that `this` is now a spantree root.
/* FP:node_flow.rs-0194 */         debug_assert!(self.span_edges[this].is_none());
/* FP:node_flow.rs-0195 */     }
/* FP:node_flow.rs-0196 */ 
/* FP:node_flow.rs-0197 */     /// Must be called exactly once for each node in the balanced-flow graph.
/* FP:node_flow.rs-0198 */     fn visit_node(&mut self, this: Node) {
/* FP:node_flow.rs-0199 */         // Assert that this node was unvisited, and mark it visited.
/* FP:node_flow.rs-0200 */         assert!(self.is_unvisited.remove(this), "node has already been visited: {this:?}");
/* FP:node_flow.rs-0201 */ 
/* FP:node_flow.rs-0202 */         // Get the supernode containing `this`, and make it the root of its
/* FP:node_flow.rs-0203 */         // component of the spantree.
/* FP:node_flow.rs-0204 */         let this_supernode = self.supernodes[this];
/* FP:node_flow.rs-0205 */         self.yank_to_spantree_root(this_supernode);
/* FP:node_flow.rs-0206 */ 
/* FP:node_flow.rs-0207 */         // Get the supernode containing all of this's successors.
/* FP:node_flow.rs-0208 */         let succ_supernode = self.succ_supernodes[this];
/* FP:node_flow.rs-0209 */         debug_assert!(self.is_supernode(succ_supernode));
/* FP:node_flow.rs-0210 */ 
/* FP:node_flow.rs-0211 */         // If two supernodes are already connected in the spantree, they will
/* FP:node_flow.rs-0212 */         // have the same spantree root. (Each supernode is connected to itself.)
/* FP:node_flow.rs-0213 */         if this_supernode != self.spantree_root(succ_supernode) {
/* FP:node_flow.rs-0214 */             // Adding this node's flow edge to the spantree would cause two
/* FP:node_flow.rs-0215 */             // previously-disconnected supernodes to become connected, so add
/* FP:node_flow.rs-0216 */             // it. That spantree-edge is now "claimed" by this node.
/* FP:node_flow.rs-0217 */             //
/* FP:node_flow.rs-0218 */             // Claiming a spantree-edge means that this node will get a counter
/* FP:node_flow.rs-0219 */             // expression instead of a physical counter. That expression is
/* FP:node_flow.rs-0220 */             // currently empty, but will be built incrementally as the other
/* FP:node_flow.rs-0221 */             // nodes are visited.
/* FP:node_flow.rs-0222 */             self.span_edges[this_supernode] = Some(SpantreeEdge {
/* FP:node_flow.rs-0223 */                 is_reversed: false,
/* FP:node_flow.rs-0224 */                 claiming_node: this,
/* FP:node_flow.rs-0225 */                 span_parent: succ_supernode,
/* FP:node_flow.rs-0226 */             });
/* FP:node_flow.rs-0227 */         } else {
/* FP:node_flow.rs-0228 */             // This node's flow edge would join two supernodes that are already
/* FP:node_flow.rs-0229 */             // connected in the spantree (or are the same supernode). That would
/* FP:node_flow.rs-0230 */             // create a cycle in the spantree, so don't add an edge.
/* FP:node_flow.rs-0231 */             //
/* FP:node_flow.rs-0232 */             // Instead, create a physical counter for this node, and add that
/* FP:node_flow.rs-0233 */             // counter to all expressions on the path from `succ_supernode` to
/* FP:node_flow.rs-0234 */             // `this_supernode`.
/* FP:node_flow.rs-0235 */ 
/* FP:node_flow.rs-0236 */             // Instead of setting `this.measure = true` as in the original paper,
/* FP:node_flow.rs-0237 */             // we just add the node's ID to its own list of terms.
/* FP:node_flow.rs-0238 */             self.counter_terms[this].push(CounterTerm { node: this, op: Op::Add });
/* FP:node_flow.rs-0239 */ 
/* FP:node_flow.rs-0240 */             // Walk the spantree from `this.successor` back to `this`. For each
/* FP:node_flow.rs-0241 */             // spantree edge along the way, add this node's physical counter to
/* FP:node_flow.rs-0242 */             // the counter expression of the node that claimed the spantree edge.
/* FP:node_flow.rs-0243 */             let mut curr = succ_supernode;
/* FP:node_flow.rs-0244 */             while curr != this_supernode {
/* FP:node_flow.rs-0245 */                 let &SpantreeEdge { is_reversed, claiming_node, span_parent } =
/* FP:node_flow.rs-0246 */                     self.span_edges[curr].as_ref().unwrap();
/* FP:node_flow.rs-0247 */                 let op = if is_reversed { Op::Subtract } else { Op::Add };
/* FP:node_flow.rs-0248 */                 self.counter_terms[claiming_node].push(CounterTerm { node: this, op });
/* FP:node_flow.rs-0249 */ 
/* FP:node_flow.rs-0250 */                 curr = span_parent;
/* FP:node_flow.rs-0251 */             }
/* FP:node_flow.rs-0252 */         }
/* FP:node_flow.rs-0253 */     }
/* FP:node_flow.rs-0254 */ 
/* FP:node_flow.rs-0255 */     /// Asserts that all nodes have been visited, and returns the computed
/* FP:node_flow.rs-0256 */     /// counter expressions (made up of physical counters) for each node.
/* FP:node_flow.rs-0257 */     fn finish(self) -> IndexVec<Node, Vec<CounterTerm<Node>>> {
/* FP:node_flow.rs-0258 */         let Self { ref span_edges, ref is_unvisited, ref counter_terms, .. } = self;
/* FP:node_flow.rs-0259 */         assert!(is_unvisited.is_empty(), "some nodes were never visited: {is_unvisited:?}");
/* FP:node_flow.rs-0260 */         debug_assert!(
/* FP:node_flow.rs-0261 */             span_edges
/* FP:node_flow.rs-0262 */                 .iter_enumerated()
/* FP:node_flow.rs-0263 */                 .all(|(node, span_edge)| { span_edge.is_some() <= self.is_supernode(node) }),
/* FP:node_flow.rs-0264 */             "only supernodes can have a span edge",
/* FP:node_flow.rs-0265 */         );
/* FP:node_flow.rs-0266 */         debug_assert!(
/* FP:node_flow.rs-0267 */             counter_terms.iter().all(|terms| !terms.is_empty()),
/* FP:node_flow.rs-0268 */             "after visiting all nodes, every node should have at least one term",
/* FP:node_flow.rs-0269 */         );
/* FP:node_flow.rs-0270 */ 
/* FP:node_flow.rs-0271 */         self.counter_terms
/* FP:node_flow.rs-0272 */     }
/* FP:node_flow.rs-0273 */ }