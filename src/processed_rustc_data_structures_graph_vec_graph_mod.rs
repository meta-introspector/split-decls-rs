/* FP:mod.rs-0001 */ use crate::rustc_index::{Idx, IndexVec};
/* FP:mod.rs-0002 */ 
/* FP:mod.rs-0003 */ use crate::graph::{DirectedGraph, NumEdges, Predecessors, Successors};
/* FP:mod.rs-0004 */ 
/* FP:mod.rs-0005 */ #[cfg(test)]
/* FP:mod.rs-0007 */ 
/* FP:mod.rs-0008 */ /// A directed graph, efficient for cases where node indices are pre-existing.
/* FP:mod.rs-0009 */ ///
/* FP:mod.rs-0010 */ /// If `BR` is true, the graph will store back-references, allowing you to get predecessors.
/* FP:mod.rs-0011 */ pub struct VecGraph<N: Idx, const BR: bool = false> {
/* FP:mod.rs-0012 */     // This is basically a `HashMap<N, (Vec<N>, If<BR, Vec<N>>)>` -- a map from a node index, to
/* FP:mod.rs-0013 */     // a list of targets of outgoing edges and (if enabled) a list of sources of incoming edges.
/* FP:mod.rs-0014 */     //
/* FP:mod.rs-0015 */     // However, it is condensed into two arrays as an optimization.
/* FP:mod.rs-0016 */     //
/* FP:mod.rs-0017 */     // `node_starts[n]` is the start of the list of targets of outgoing edges for node `n`.
/* FP:mod.rs-0018 */     // So you can get node's successors with `edge_targets[node_starts[n]..node_starts[n + 1]]`.
/* FP:mod.rs-0019 */     //
/* FP:mod.rs-0020 */     // If `BR` is true (back references are enabled), then `node_starts[n + edge_count]` is the
/* FP:mod.rs-0021 */     // start of the list of *sources* of incoming edges. You can get predecessors of a node
/* FP:mod.rs-0022 */     // similarly to its successors but offsetting by `edge_count`. `edge_count` is
/* FP:mod.rs-0023 */     // `edge_targets.len()/2` (again, in case BR is true) because half of the vec is back refs.
/* FP:mod.rs-0024 */     //
/* FP:mod.rs-0025 */     // All of this might be confusing, so here is an example graph and its representation:
/* FP:mod.rs-0026 */     //
/* FP:mod.rs-0027 */     //       n3 ----+
/* FP:mod.rs-0028 */     //        ^     |                           (if BR = true)
/* FP:mod.rs-0029 */     //        |     v     outgoing edges        incoming edges
/* FP:mod.rs-0030 */     // n0 -> n1 -> n2     ______________      __________________
/* FP:mod.rs-0031 */     //                   /              \    /                  \
/* FP:mod.rs-0032 */     //  node indices[1]:  n0, n1, n2, n3,     n0, n1, n2, n3,       n/a
/* FP:mod.rs-0033 */     //      vec indices:  n0, n1, n2, n3,     n4, n5, n6, n7,       n8
/* FP:mod.rs-0034 */     //      node_starts:  [0,  1,  3,  4       4,  4,  5,  7,        8]
/* FP:mod.rs-0035 */     //                     |   |   |   |       |   |   |   |         |
/* FP:mod.rs-0036 */     //                     |   |   +---+       +---+   |   +---+     |
/* FP:mod.rs-0037 */     //                     |   |       |           |   |       |     |
/* FP:mod.rs-0038 */     //                     v   v       v           v   v       v     v
/* FP:mod.rs-0039 */     //     edge_targets: [n1, n2, n3, n2          n0, n1, n3, n1]
/* FP:mod.rs-0040 */     //                   /    \____/   |           |  \____/    \
/* FP:mod.rs-0041 */     //             n0->n1     /        |           |       \     n3<-n1
/* FP:mod.rs-0042 */     //                       /    n3->n2 [2]  n1<-n0 [2]    \
/* FP:mod.rs-0043 */     //         n1->n2, n1->n3                                n2<-n1, n2<-n3
/* FP:mod.rs-0044 */     //
/* FP:mod.rs-0045 */     // The incoming edges are basically stored in the same way as outgoing edges, but offset and
/* FP:mod.rs-0046 */     // the graph they store is the inverse of the original. Last index in the `node_starts` array
/* FP:mod.rs-0047 */     // always points to one-past-the-end, so that we don't need to bound check `node_starts[n + 1]`
/* FP:mod.rs-0048 */     //
/* FP:mod.rs-0049 */     // [1]: "node indices" are the indices a user of `VecGraph` might use,
/* FP:mod.rs-0050 */     //      note that they are different from "vec indices",
/* FP:mod.rs-0051 */     //      which are the real indices you need to index `node_starts`
/* FP:mod.rs-0052 */     //
/* FP:mod.rs-0053 */     // [2]: Note that even though n2 also points to here,
/* FP:mod.rs-0054 */     //      the next index also points here, so n2 has no
/* FP:mod.rs-0055 */     //      successors (`edge_targets[3..3] = []`).
/* FP:mod.rs-0056 */     //      Similarly with n0 and incoming edges
/* FP:mod.rs-0057 */     //
/* FP:mod.rs-0058 */     // If this is still confusing... then sorry :(
/* FP:mod.rs-0059 */     //
/* FP:mod.rs-0060 */     /// Indices into `edge_targets` that signify a start of list of edges.
/* FP:mod.rs-0061 */     node_starts: IndexVec<N, usize>,
/* FP:mod.rs-0062 */ 
/* FP:mod.rs-0063 */     /// Targets (or sources for back refs) of edges
/* FP:mod.rs-0064 */     edge_targets: Vec<N>,
/* FP:mod.rs-0065 */ }
/* FP:mod.rs-0066 */ 
/* FP:mod.rs-0067 */ impl<N: Idx + Ord, const BR: bool> VecGraph<N, BR> {
/* FP:mod.rs-0068 */     pub fn new(num_nodes: usize, mut edge_pairs: Vec<(N, N)>) -> Self {
/* FP:mod.rs-0069 */         let num_edges = edge_pairs.len();
/* FP:mod.rs-0070 */ 
/* FP:mod.rs-0071 */         let nodes_cap = match BR {
/* FP:mod.rs-0072 */             // +1 for special entry at the end, pointing one past the end of `edge_targets`
/* FP:mod.rs-0073 */             false => num_nodes + 1,
/* FP:mod.rs-0074 */             // *2 for back references
/* FP:mod.rs-0075 */             true => (num_nodes * 2) + 1,
/* FP:mod.rs-0076 */         };
/* FP:mod.rs-0077 */ 
/* FP:mod.rs-0078 */         let edges_cap = match BR {
/* FP:mod.rs-0079 */             false => num_edges,
/* FP:mod.rs-0080 */             // *2 for back references
/* FP:mod.rs-0081 */             true => num_edges * 2,
/* FP:mod.rs-0082 */         };
/* FP:mod.rs-0083 */ 
/* FP:mod.rs-0084 */         let mut node_starts = IndexVec::with_capacity(nodes_cap);
/* FP:mod.rs-0085 */         let mut edge_targets = Vec::with_capacity(edges_cap);
/* FP:mod.rs-0086 */ 
/* FP:mod.rs-0087 */         // Sort the edges by the source -- this is important.
/* FP:mod.rs-0088 */         edge_pairs.sort();
/* FP:mod.rs-0089 */ 
/* FP:mod.rs-0090 */         // Fill forward references
/* FP:mod.rs-0091 */         create_index(
/* FP:mod.rs-0092 */             num_nodes,
/* FP:mod.rs-0093 */             &mut edge_pairs.iter().map(|&(src, _)| src),
/* FP:mod.rs-0094 */             &mut edge_pairs.iter().map(|&(_, tgt)| tgt),
/* FP:mod.rs-0095 */             &mut edge_targets,
/* FP:mod.rs-0096 */             &mut node_starts,
/* FP:mod.rs-0097 */         );
/* FP:mod.rs-0098 */ 
/* FP:mod.rs-0099 */         // Fill back references
/* FP:mod.rs-0100 */         if BR {
/* FP:mod.rs-0101 */             // Pop the special "last" entry, it will be replaced by first back ref
/* FP:mod.rs-0102 */             node_starts.pop();
/* FP:mod.rs-0103 */ 
/* FP:mod.rs-0104 */             // Re-sort the edges so that they are sorted by target
/* FP:mod.rs-0105 */             edge_pairs.sort_by_key(|&(src, tgt)| (tgt, src));
/* FP:mod.rs-0106 */ 
/* FP:mod.rs-0107 */             create_index(
/* FP:mod.rs-0108 */                 // Back essentially double the number of nodes
/* FP:mod.rs-0109 */                 num_nodes * 2,
/* FP:mod.rs-0110 */                 // NB: the source/target are switched here too
/* FP:mod.rs-0111 */                 // NB: we double the key index, so that we can later use *2 to get the back references
/* FP:mod.rs-0112 */                 &mut edge_pairs.iter().map(|&(_, tgt)| N::new(tgt.index() + num_nodes)),
/* FP:mod.rs-0113 */                 &mut edge_pairs.iter().map(|&(src, _)| src),
/* FP:mod.rs-0114 */                 &mut edge_targets,
/* FP:mod.rs-0115 */                 &mut node_starts,
/* FP:mod.rs-0116 */             );
/* FP:mod.rs-0117 */         }
/* FP:mod.rs-0118 */ 
/* FP:mod.rs-0119 */         Self { node_starts, edge_targets }
/* FP:mod.rs-0120 */     }
/* FP:mod.rs-0121 */ 
/* FP:mod.rs-0122 */     /// Gets the successors for `source` as a slice.
/* FP:mod.rs-0123 */     pub fn successors(&self, source: N) -> &[N] {
/* FP:mod.rs-0124 */         assert!(source.index() < self.num_nodes());
/* FP:mod.rs-0125 */ 
/* FP:mod.rs-0126 */         let start_index = self.node_starts[source];
/* FP:mod.rs-0127 */         let end_index = self.node_starts[source.plus(1)];
/* FP:mod.rs-0128 */         &self.edge_targets[start_index..end_index]
/* FP:mod.rs-0129 */     }
/* FP:mod.rs-0130 */ }
/* FP:mod.rs-0131 */ 
/* FP:mod.rs-0132 */ impl<N: Idx + Ord> VecGraph<N, true> {
/* FP:mod.rs-0133 */     /// Gets the predecessors for `target` as a slice.
/* FP:mod.rs-0134 */     pub fn predecessors(&self, target: N) -> &[N] {
/* FP:mod.rs-0135 */         assert!(target.index() < self.num_nodes());
/* FP:mod.rs-0136 */ 
/* FP:mod.rs-0137 */         let target = N::new(target.index() + self.num_nodes());
/* FP:mod.rs-0138 */ 
/* FP:mod.rs-0139 */         let start_index = self.node_starts[target];
/* FP:mod.rs-0140 */         let end_index = self.node_starts[target.plus(1)];
/* FP:mod.rs-0141 */         &self.edge_targets[start_index..end_index]
/* FP:mod.rs-0142 */     }
/* FP:mod.rs-0143 */ }
/* FP:mod.rs-0144 */ 
/* FP:mod.rs-0145 */ /// Creates/initializes the index for the [`VecGraph`]. A helper for [`VecGraph::new`].
/* FP:mod.rs-0146 */ ///
/* FP:mod.rs-0147 */ /// - `num_nodes` is the target number of nodes in the graph
/* FP:mod.rs-0148 */ /// - `sorted_edge_sources` are the edge sources, sorted
/* FP:mod.rs-0149 */ /// - `associated_edge_targets` are the edge *targets* in the same order as sources
/* FP:mod.rs-0150 */ /// - `edge_targets` is the vec of targets to be extended
/* FP:mod.rs-0151 */ /// - `node_starts` is the index to be filled
/* FP:mod.rs-0152 */ fn create_index<N: Idx + Ord>(
/* FP:mod.rs-0153 */     num_nodes: usize,
/* FP:mod.rs-0154 */     sorted_edge_sources: &mut dyn Iterator<Item = N>,
/* FP:mod.rs-0155 */     associated_edge_targets: &mut dyn Iterator<Item = N>,
/* FP:mod.rs-0156 */     edge_targets: &mut Vec<N>,
/* FP:mod.rs-0157 */     node_starts: &mut IndexVec<N, usize>,
/* FP:mod.rs-0158 */ ) {
/* FP:mod.rs-0159 */     let offset = edge_targets.len();
/* FP:mod.rs-0160 */ 
/* FP:mod.rs-0161 */     // Store the *target* of each edge into `edge_targets`.
/* FP:mod.rs-0162 */     edge_targets.extend(associated_edge_targets);
/* FP:mod.rs-0163 */ 
/* FP:mod.rs-0164 */     // Create the *edge starts* array. We are iterating over the
/* FP:mod.rs-0165 */     // (sorted) edge pairs. We maintain the invariant that the
/* FP:mod.rs-0166 */     // length of the `node_starts` array is enough to store the
/* FP:mod.rs-0167 */     // current source node -- so when we see that the source node
/* FP:mod.rs-0168 */     // for an edge is greater than the current length, we grow the
/* FP:mod.rs-0169 */     // edge-starts array by just enough.
/* FP:mod.rs-0170 */     for (index, source) in sorted_edge_sources.enumerate() {
/* FP:mod.rs-0171 */         // If we have a list like `[(0, x), (2, y)]`:
/* FP:mod.rs-0172 */         //
/* FP:mod.rs-0173 */         // - Start out with `node_starts` of `[]`
/* FP:mod.rs-0174 */         // - Iterate to `(0, x)` at index 0:
/* FP:mod.rs-0175 */         //   - Push one entry because `node_starts.len()` (0) is <= the source (0)
/* FP:mod.rs-0176 */         //   - Leaving us with `node_starts` of `[0]`
/* FP:mod.rs-0177 */         // - Iterate to `(2, y)` at index 1:
/* FP:mod.rs-0178 */         //   - Push one entry because `node_starts.len()` (1) is <= the source (2)
/* FP:mod.rs-0179 */         //   - Push one entry because `node_starts.len()` (2) is <= the source (2)
/* FP:mod.rs-0180 */         //   - Leaving us with `node_starts` of `[0, 1, 1]`
/* FP:mod.rs-0181 */         // - Loop terminates
/* FP:mod.rs-0182 */         while node_starts.len() <= source.index() {
/* FP:mod.rs-0183 */             node_starts.push(index + offset);
/* FP:mod.rs-0184 */         }
/* FP:mod.rs-0185 */     }
/* FP:mod.rs-0186 */ 
/* FP:mod.rs-0187 */     // Pad out the `node_starts` array so that it has `num_nodes +
/* FP:mod.rs-0188 */     // 1` entries. Continuing our example above, if `num_nodes` is
/* FP:mod.rs-0189 */     // be `3`, we would push one more index: `[0, 1, 1, 2]`.
/* FP:mod.rs-0190 */     //
/* FP:mod.rs-0191 */     // Interpretation of that vector:
/* FP:mod.rs-0192 */     //
/* FP:mod.rs-0193 */     // [0, 1, 1, 2]
/* FP:mod.rs-0194 */     //        ---- range for N=2
/* FP:mod.rs-0195 */     //     ---- range for N=1
/* FP:mod.rs-0196 */     //  ---- range for N=0
/* FP:mod.rs-0197 */     while node_starts.len() <= num_nodes {
/* FP:mod.rs-0198 */         node_starts.push(edge_targets.len());
/* FP:mod.rs-0199 */     }
/* FP:mod.rs-0200 */ 
/* FP:mod.rs-0201 */     assert_eq!(node_starts.len(), num_nodes + 1);
/* FP:mod.rs-0202 */ }
/* FP:mod.rs-0203 */ 
/* FP:mod.rs-0204 */ impl<N: Idx, const BR: bool> DirectedGraph for VecGraph<N, BR> {
/* FP:mod.rs-0205 */     type Node = N;
/* FP:mod.rs-0206 */ 
/* FP:mod.rs-0207 */     fn num_nodes(&self) -> usize {
/* FP:mod.rs-0208 */         match BR {
/* FP:mod.rs-0209 */             false => self.node_starts.len() - 1,
/* FP:mod.rs-0210 */             // If back refs are enabled, half of the array is said back refs
/* FP:mod.rs-0211 */             true => (self.node_starts.len() - 1) / 2,
/* FP:mod.rs-0212 */         }
/* FP:mod.rs-0213 */     }
/* FP:mod.rs-0214 */ }
/* FP:mod.rs-0215 */ 
/* FP:mod.rs-0216 */ impl<N: Idx, const BR: bool> NumEdges for VecGraph<N, BR> {
/* FP:mod.rs-0217 */     fn num_edges(&self) -> usize {
/* FP:mod.rs-0218 */         match BR {
/* FP:mod.rs-0219 */             false => self.edge_targets.len(),
/* FP:mod.rs-0220 */             // If back refs are enabled, half of the array is reversed edges for them
/* FP:mod.rs-0221 */             true => self.edge_targets.len() / 2,
/* FP:mod.rs-0222 */         }
/* FP:mod.rs-0223 */     }
/* FP:mod.rs-0224 */ }
/* FP:mod.rs-0225 */ 
/* FP:mod.rs-0226 */ impl<N: Idx + Ord, const BR: bool> Successors for VecGraph<N, BR> {
/* FP:mod.rs-0227 */     fn successors(&self, node: N) -> impl Iterator<Item = Self::Node> {
/* FP:mod.rs-0228 */         self.successors(node).iter().cloned()
/* FP:mod.rs-0229 */     }
/* FP:mod.rs-0230 */ }
/* FP:mod.rs-0231 */ 
/* FP:mod.rs-0232 */ impl<N: Idx + Ord> Predecessors for VecGraph<N, true> {
/* FP:mod.rs-0233 */     fn predecessors(&self, node: Self::Node) -> impl Iterator<Item = Self::Node> {
/* FP:mod.rs-0234 */         self.predecessors(node).iter().cloned()
/* FP:mod.rs-0235 */     }
/* FP:mod.rs-0236 */ }