/* FP:mod.rs-0001 */ // See [`LinkedGraph`].
/* FP:mod.rs-0002 */ //
/* FP:mod.rs-0003 */ // # Interface details
/* FP:mod.rs-0004 */ //
/* FP:mod.rs-0005 */ // You customize the graph by specifying a "node data" type `N` and an
/* FP:mod.rs-0006 */ // "edge data" type `E`. You can then later gain access (mutable or
/* FP:mod.rs-0007 */ // immutable) to these "user-data" bits. Currently, you can only add
/* FP:mod.rs-0008 */ // nodes or edges to the graph. You cannot remove or modify them once
/* FP:mod.rs-0009 */ // added. This could be changed if we have a need.
/* FP:mod.rs-0010 */ //
/* FP:mod.rs-0011 */ // # Implementation details
/* FP:mod.rs-0012 */ //
/* FP:mod.rs-0013 */ // The main tricky thing about this code is the way that edges are
/* FP:mod.rs-0014 */ // stored. The edges are stored in a central array, but they are also
/* FP:mod.rs-0015 */ // threaded onto two linked lists for each node, one for incoming edges
/* FP:mod.rs-0016 */ // and one for outgoing edges. Note that every edge is a member of some
/* FP:mod.rs-0017 */ // incoming list and some outgoing list. Basically you can load the
/* FP:mod.rs-0018 */ // first index of the linked list from the node data structures (the
/* FP:mod.rs-0019 */ // field `first_edge`) and then, for each edge, load the next index from
/* FP:mod.rs-0020 */ // the field `next_edge`). Each of those fields is an array that should
/* FP:mod.rs-0021 */ // be indexed by the direction (see the type `Direction`).
/* FP:mod.rs-0022 */ 
/* FP:mod.rs-0023 */ use std::fmt::Debug;
/* FP:mod.rs-0024 */ 
/* FP:mod.rs-0025 */ use crate::rustc_index::bit_set::DenseBitSet;
/* FP:mod.rs-0026 */ use tracing::debug;
/* FP:mod.rs-0027 */ 
/* FP:mod.rs-0028 */ #[cfg(test)]
/* FP:mod.rs-0030 */ 
/* FP:mod.rs-0031 */ /// A concrete graph implementation that supports:
/* FP:mod.rs-0032 */ /// - Nodes and/or edges labelled with custom data types (`N` and `E` respectively).
/* FP:mod.rs-0033 */ /// - Incremental addition of new nodes/edges (but not removal).
/* FP:mod.rs-0034 */ /// - Flat storage of node/edge data in a pair of vectors.
/* FP:mod.rs-0035 */ /// - Iteration over any node's out-edges or in-edges, via linked lists
/* FP:mod.rs-0036 */ ///   threaded through the node/edge data.
/* FP:mod.rs-0037 */ ///
/* FP:mod.rs-0038 */ /// # Caution
/* FP:mod.rs-0039 */ /// This is an older graph implementation that is still used by some pieces
/* FP:mod.rs-0040 */ /// of diagnostic/debugging code. New code that needs a graph data structure
/* FP:mod.rs-0041 */ /// should consider using `VecGraph` instead, or implementing its own
/* FP:mod.rs-0042 */ /// special-purpose graph with the specific features needed.
/* FP:mod.rs-0043 */ ///
/* FP:mod.rs-0044 */ /// This graph implementation predates the later [graph traits](crate::graph),
/* FP:mod.rs-0045 */ /// and does not implement those traits, so it has its own implementations of a
/* FP:mod.rs-0046 */ /// few basic graph algorithms.
/* FP:mod.rs-0047 */ pub struct LinkedGraph<N, E> {
/* FP:mod.rs-0048 */     nodes: Vec<Node<N>>,
/* FP:mod.rs-0049 */     edges: Vec<Edge<E>>,
/* FP:mod.rs-0050 */ }
/* FP:mod.rs-0051 */ 
/* FP:mod.rs-0052 */ pub struct Node<N> {
/* FP:mod.rs-0053 */     first_edge: [EdgeIndex; 2], // see module comment
/* FP:mod.rs-0054 */     pub data: N,
/* FP:mod.rs-0055 */ }
/* FP:mod.rs-0056 */ 
/* FP:mod.rs-0057 */ #[derive(Debug)]
/* FP:mod.rs-0058 */ pub struct Edge<E> {
/* FP:mod.rs-0059 */     next_edge: [EdgeIndex; 2], // see module comment
/* FP:mod.rs-0060 */     source: NodeIndex,
/* FP:mod.rs-0061 */     target: NodeIndex,
/* FP:mod.rs-0062 */     pub data: E,
/* FP:mod.rs-0063 */ }
/* FP:mod.rs-0064 */ 
/* FP:mod.rs-0065 */ #[derive(Copy, Clone, PartialEq, Debug)]
/* FP:mod.rs-0066 */ pub struct NodeIndex(pub usize);
/* FP:mod.rs-0067 */ 
/* FP:mod.rs-0068 */ #[derive(Copy, Clone, PartialEq, Debug)]
/* FP:mod.rs-0069 */ pub struct EdgeIndex(pub usize);
/* FP:mod.rs-0070 */ 
/* FP:mod.rs-0071 */ pub const INVALID_EDGE_INDEX: EdgeIndex = EdgeIndex(usize::MAX);
/* FP:mod.rs-0072 */ 
/* FP:mod.rs-0073 */ // Use a private field here to guarantee no more instances are created:
/* FP:mod.rs-0074 */ #[derive(Copy, Clone, Debug, PartialEq)]
/* FP:mod.rs-0075 */ pub struct Direction {
/* FP:mod.rs-0076 */     repr: usize,
/* FP:mod.rs-0077 */ }
/* FP:mod.rs-0078 */ 
/* FP:mod.rs-0079 */ pub const OUTGOING: Direction = Direction { repr: 0 };
/* FP:mod.rs-0080 */ 
/* FP:mod.rs-0081 */ pub const INCOMING: Direction = Direction { repr: 1 };
/* FP:mod.rs-0082 */ 
/* FP:mod.rs-0083 */ impl NodeIndex {
/* FP:mod.rs-0084 */     /// Returns unique ID (unique with respect to the graph holding associated node).
/* FP:mod.rs-0085 */     pub fn node_id(self) -> usize {
/* FP:mod.rs-0086 */         self.0
/* FP:mod.rs-0087 */     }
/* FP:mod.rs-0088 */ }
/* FP:mod.rs-0089 */ 
/* FP:mod.rs-0090 */ impl<N: Debug, E: Debug> LinkedGraph<N, E> {
/* FP:mod.rs-0091 */     pub fn new() -> Self {
/* FP:mod.rs-0092 */         Self { nodes: Vec::new(), edges: Vec::new() }
/* FP:mod.rs-0093 */     }
/* FP:mod.rs-0094 */ 
/* FP:mod.rs-0095 */     pub fn with_capacity(nodes: usize, edges: usize) -> Self {
/* FP:mod.rs-0096 */         Self { nodes: Vec::with_capacity(nodes), edges: Vec::with_capacity(edges) }
/* FP:mod.rs-0097 */     }
/* FP:mod.rs-0098 */ 
/* FP:mod.rs-0099 */     // # Simple accessors
/* FP:mod.rs-0100 */ 
/* FP:mod.rs-0101 */     #[inline]
/* FP:mod.rs-0102 */     pub fn all_nodes(&self) -> &[Node<N>] {
/* FP:mod.rs-0103 */         &self.nodes
/* FP:mod.rs-0104 */     }
/* FP:mod.rs-0105 */ 
/* FP:mod.rs-0106 */     #[inline]
/* FP:mod.rs-0107 */     pub fn len_nodes(&self) -> usize {
/* FP:mod.rs-0108 */         self.nodes.len()
/* FP:mod.rs-0109 */     }
/* FP:mod.rs-0110 */ 
/* FP:mod.rs-0111 */     #[inline]
/* FP:mod.rs-0112 */     pub fn all_edges(&self) -> &[Edge<E>] {
/* FP:mod.rs-0113 */         &self.edges
/* FP:mod.rs-0114 */     }
/* FP:mod.rs-0115 */ 
/* FP:mod.rs-0116 */     #[inline]
/* FP:mod.rs-0117 */     pub fn len_edges(&self) -> usize {
/* FP:mod.rs-0118 */         self.edges.len()
/* FP:mod.rs-0119 */     }
/* FP:mod.rs-0120 */ 
/* FP:mod.rs-0121 */     // # Node construction
/* FP:mod.rs-0122 */ 
/* FP:mod.rs-0123 */     pub fn next_node_index(&self) -> NodeIndex {
/* FP:mod.rs-0124 */         NodeIndex(self.nodes.len())
/* FP:mod.rs-0125 */     }
/* FP:mod.rs-0126 */ 
/* FP:mod.rs-0127 */     pub fn add_node(&mut self, data: N) -> NodeIndex {
/* FP:mod.rs-0128 */         let idx = self.next_node_index();
/* FP:mod.rs-0129 */         self.nodes.push(Node { first_edge: [INVALID_EDGE_INDEX, INVALID_EDGE_INDEX], data });
/* FP:mod.rs-0130 */         idx
/* FP:mod.rs-0131 */     }
/* FP:mod.rs-0132 */ 
/* FP:mod.rs-0133 */     pub fn mut_node_data(&mut self, idx: NodeIndex) -> &mut N {
/* FP:mod.rs-0134 */         &mut self.nodes[idx.0].data
/* FP:mod.rs-0135 */     }
/* FP:mod.rs-0136 */ 
/* FP:mod.rs-0137 */     pub fn node_data(&self, idx: NodeIndex) -> &N {
/* FP:mod.rs-0138 */         &self.nodes[idx.0].data
/* FP:mod.rs-0139 */     }
/* FP:mod.rs-0140 */ 
/* FP:mod.rs-0141 */     pub fn node(&self, idx: NodeIndex) -> &Node<N> {
/* FP:mod.rs-0142 */         &self.nodes[idx.0]
/* FP:mod.rs-0143 */     }
/* FP:mod.rs-0144 */ 
/* FP:mod.rs-0145 */     // # Edge construction and queries
/* FP:mod.rs-0146 */ 
/* FP:mod.rs-0147 */     pub fn next_edge_index(&self) -> EdgeIndex {
/* FP:mod.rs-0148 */         EdgeIndex(self.edges.len())
/* FP:mod.rs-0149 */     }
/* FP:mod.rs-0150 */ 
/* FP:mod.rs-0151 */     pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, data: E) -> EdgeIndex {
/* FP:mod.rs-0152 */         debug!("graph: add_edge({:?}, {:?}, {:?})", source, target, data);
/* FP:mod.rs-0153 */ 
/* FP:mod.rs-0154 */         let idx = self.next_edge_index();
/* FP:mod.rs-0155 */ 
/* FP:mod.rs-0156 */         // read current first of the list of edges from each node
/* FP:mod.rs-0157 */         let source_first = self.nodes[source.0].first_edge[OUTGOING.repr];
/* FP:mod.rs-0158 */         let target_first = self.nodes[target.0].first_edge[INCOMING.repr];
/* FP:mod.rs-0159 */ 
/* FP:mod.rs-0160 */         // create the new edge, with the previous firsts from each node
/* FP:mod.rs-0161 */         // as the next pointers
/* FP:mod.rs-0162 */         self.edges.push(Edge { next_edge: [source_first, target_first], source, target, data });
/* FP:mod.rs-0163 */ 
/* FP:mod.rs-0164 */         // adjust the firsts for each node target be the next object.
/* FP:mod.rs-0165 */         self.nodes[source.0].first_edge[OUTGOING.repr] = idx;
/* FP:mod.rs-0166 */         self.nodes[target.0].first_edge[INCOMING.repr] = idx;
/* FP:mod.rs-0167 */ 
/* FP:mod.rs-0168 */         idx
/* FP:mod.rs-0169 */     }
/* FP:mod.rs-0170 */ 
/* FP:mod.rs-0171 */     pub fn edge(&self, idx: EdgeIndex) -> &Edge<E> {
/* FP:mod.rs-0172 */         &self.edges[idx.0]
/* FP:mod.rs-0173 */     }
/* FP:mod.rs-0174 */ 
/* FP:mod.rs-0175 */     // # Iterating over nodes, edges
/* FP:mod.rs-0176 */ 
/* FP:mod.rs-0177 */     pub fn enumerated_nodes(&self) -> impl Iterator<Item = (NodeIndex, &Node<N>)> {
/* FP:mod.rs-0178 */         self.nodes.iter().enumerate().map(|(idx, n)| (NodeIndex(idx), n))
/* FP:mod.rs-0179 */     }
/* FP:mod.rs-0180 */ 
/* FP:mod.rs-0181 */     pub fn enumerated_edges(&self) -> impl Iterator<Item = (EdgeIndex, &Edge<E>)> {
/* FP:mod.rs-0182 */         self.edges.iter().enumerate().map(|(idx, e)| (EdgeIndex(idx), e))
/* FP:mod.rs-0183 */     }
/* FP:mod.rs-0184 */ 
/* FP:mod.rs-0185 */     pub fn each_node<'a>(&'a self, mut f: impl FnMut(NodeIndex, &'a Node<N>) -> bool) -> bool {
/* FP:mod.rs-0186 */         // Iterates over all edges defined in the graph.
/* FP:mod.rs-0187 */         self.enumerated_nodes().all(|(node_idx, node)| f(node_idx, node))
/* FP:mod.rs-0188 */     }
/* FP:mod.rs-0189 */ 
/* FP:mod.rs-0190 */     pub fn each_edge<'a>(&'a self, mut f: impl FnMut(EdgeIndex, &'a Edge<E>) -> bool) -> bool {
/* FP:mod.rs-0191 */         // Iterates over all edges defined in the graph
/* FP:mod.rs-0192 */         self.enumerated_edges().all(|(edge_idx, edge)| f(edge_idx, edge))
/* FP:mod.rs-0193 */     }
/* FP:mod.rs-0194 */ 
/* FP:mod.rs-0195 */     pub fn outgoing_edges(&self, source: NodeIndex) -> AdjacentEdges<'_, N, E> {
/* FP:mod.rs-0196 */         self.adjacent_edges(source, OUTGOING)
/* FP:mod.rs-0197 */     }
/* FP:mod.rs-0198 */ 
/* FP:mod.rs-0199 */     pub fn incoming_edges(&self, source: NodeIndex) -> AdjacentEdges<'_, N, E> {
/* FP:mod.rs-0200 */         self.adjacent_edges(source, INCOMING)
/* FP:mod.rs-0201 */     }
/* FP:mod.rs-0202 */ 
/* FP:mod.rs-0203 */     pub fn adjacent_edges(
/* FP:mod.rs-0204 */         &self,
/* FP:mod.rs-0205 */         source: NodeIndex,
/* FP:mod.rs-0206 */         direction: Direction,
/* FP:mod.rs-0207 */     ) -> AdjacentEdges<'_, N, E> {
/* FP:mod.rs-0208 */         let first_edge = self.node(source).first_edge[direction.repr];
/* FP:mod.rs-0209 */         AdjacentEdges { graph: self, direction, next: first_edge }
/* FP:mod.rs-0210 */     }
/* FP:mod.rs-0211 */ 
/* FP:mod.rs-0212 */     pub fn successor_nodes(&self, source: NodeIndex) -> impl Iterator<Item = NodeIndex> {
/* FP:mod.rs-0213 */         self.outgoing_edges(source).targets()
/* FP:mod.rs-0214 */     }
/* FP:mod.rs-0215 */ 
/* FP:mod.rs-0216 */     pub fn predecessor_nodes(&self, target: NodeIndex) -> impl Iterator<Item = NodeIndex> {
/* FP:mod.rs-0217 */         self.incoming_edges(target).sources()
/* FP:mod.rs-0218 */     }
/* FP:mod.rs-0219 */ 
/* FP:mod.rs-0220 */     pub fn depth_traverse(
/* FP:mod.rs-0221 */         &self,
/* FP:mod.rs-0222 */         start: NodeIndex,
/* FP:mod.rs-0223 */         direction: Direction,
/* FP:mod.rs-0224 */     ) -> DepthFirstTraversal<'_, N, E> {
/* FP:mod.rs-0225 */         DepthFirstTraversal::with_start_node(self, start, direction)
/* FP:mod.rs-0226 */     }
/* FP:mod.rs-0227 */ 
/* FP:mod.rs-0228 */     pub fn nodes_in_postorder(
/* FP:mod.rs-0229 */         &self,
/* FP:mod.rs-0230 */         direction: Direction,
/* FP:mod.rs-0231 */         entry_node: NodeIndex,
/* FP:mod.rs-0232 */     ) -> Vec<NodeIndex> {
/* FP:mod.rs-0233 */         let mut visited = DenseBitSet::new_empty(self.len_nodes());
/* FP:mod.rs-0234 */         let mut stack = vec![];
/* FP:mod.rs-0235 */         let mut result = Vec::with_capacity(self.len_nodes());
/* FP:mod.rs-0236 */         let mut push_node = |stack: &mut Vec<_>, node: NodeIndex| {
/* FP:mod.rs-0237 */             if visited.insert(node.0) {
/* FP:mod.rs-0238 */                 stack.push((node, self.adjacent_edges(node, direction)));
/* FP:mod.rs-0239 */             }
/* FP:mod.rs-0240 */         };
/* FP:mod.rs-0241 */ 
/* FP:mod.rs-0242 */         for node in
/* FP:mod.rs-0243 */             Some(entry_node).into_iter().chain(self.enumerated_nodes().map(|(node, _)| node))
/* FP:mod.rs-0244 */         {
/* FP:mod.rs-0245 */             push_node(&mut stack, node);
/* FP:mod.rs-0246 */             while let Some((node, mut iter)) = stack.pop() {
/* FP:mod.rs-0247 */                 if let Some((_, child)) = iter.next() {
/* FP:mod.rs-0248 */                     let target = child.source_or_target(direction);
/* FP:mod.rs-0249 */                     // the current node needs more processing, so
/* FP:mod.rs-0250 */                     // add it back to the stack
/* FP:mod.rs-0251 */                     stack.push((node, iter));
/* FP:mod.rs-0252 */                     // and then push the new node
/* FP:mod.rs-0253 */                     push_node(&mut stack, target);
/* FP:mod.rs-0254 */                 } else {
/* FP:mod.rs-0255 */                     result.push(node);
/* FP:mod.rs-0256 */                 }
/* FP:mod.rs-0257 */             }
/* FP:mod.rs-0258 */         }
/* FP:mod.rs-0259 */ 
/* FP:mod.rs-0260 */         assert_eq!(result.len(), self.len_nodes());
/* FP:mod.rs-0261 */         result
/* FP:mod.rs-0262 */     }
/* FP:mod.rs-0263 */ }
/* FP:mod.rs-0264 */ 
/* FP:mod.rs-0265 */ // # Iterators
/* FP:mod.rs-0266 */ 
/* FP:mod.rs-0267 */ pub struct AdjacentEdges<'g, N, E> {
/* FP:mod.rs-0268 */     graph: &'g LinkedGraph<N, E>,
/* FP:mod.rs-0269 */     direction: Direction,
/* FP:mod.rs-0270 */     next: EdgeIndex,
/* FP:mod.rs-0271 */ }
/* FP:mod.rs-0272 */ 
/* FP:mod.rs-0273 */ impl<'g, N: Debug, E: Debug> AdjacentEdges<'g, N, E> {
/* FP:mod.rs-0274 */     fn targets(self) -> impl Iterator<Item = NodeIndex> {
/* FP:mod.rs-0275 */         self.map(|(_, edge)| edge.target)
/* FP:mod.rs-0276 */     }
/* FP:mod.rs-0277 */ 
/* FP:mod.rs-0278 */     fn sources(self) -> impl Iterator<Item = NodeIndex> {
/* FP:mod.rs-0279 */         self.map(|(_, edge)| edge.source)
/* FP:mod.rs-0280 */     }
/* FP:mod.rs-0281 */ }
/* FP:mod.rs-0282 */ 
/* FP:mod.rs-0283 */ impl<'g, N: Debug, E: Debug> Iterator for AdjacentEdges<'g, N, E> {
/* FP:mod.rs-0284 */     type Item = (EdgeIndex, &'g Edge<E>);
/* FP:mod.rs-0285 */ 
/* FP:mod.rs-0286 */     fn next(&mut self) -> Option<(EdgeIndex, &'g Edge<E>)> {
/* FP:mod.rs-0287 */         let edge_index = self.next;
/* FP:mod.rs-0288 */         if edge_index == INVALID_EDGE_INDEX {
/* FP:mod.rs-0289 */             return None;
/* FP:mod.rs-0290 */         }
/* FP:mod.rs-0291 */ 
/* FP:mod.rs-0292 */         let edge = self.graph.edge(edge_index);
/* FP:mod.rs-0293 */         self.next = edge.next_edge[self.direction.repr];
/* FP:mod.rs-0294 */         Some((edge_index, edge))
/* FP:mod.rs-0295 */     }
/* FP:mod.rs-0296 */ 
/* FP:mod.rs-0297 */     fn size_hint(&self) -> (usize, Option<usize>) {
/* FP:mod.rs-0298 */         // At most, all the edges in the graph.
/* FP:mod.rs-0299 */         (0, Some(self.graph.len_edges()))
/* FP:mod.rs-0300 */     }
/* FP:mod.rs-0301 */ }
/* FP:mod.rs-0302 */ 
/* FP:mod.rs-0303 */ pub struct DepthFirstTraversal<'g, N, E> {
/* FP:mod.rs-0304 */     graph: &'g LinkedGraph<N, E>,
/* FP:mod.rs-0305 */     stack: Vec<NodeIndex>,
/* FP:mod.rs-0306 */     visited: DenseBitSet<usize>,
/* FP:mod.rs-0307 */     direction: Direction,
/* FP:mod.rs-0308 */ }
/* FP:mod.rs-0309 */ 
/* FP:mod.rs-0310 */ impl<'g, N: Debug, E: Debug> DepthFirstTraversal<'g, N, E> {
/* FP:mod.rs-0311 */     pub fn with_start_node(
/* FP:mod.rs-0312 */         graph: &'g LinkedGraph<N, E>,
/* FP:mod.rs-0313 */         start_node: NodeIndex,
/* FP:mod.rs-0314 */         direction: Direction,
/* FP:mod.rs-0315 */     ) -> Self {
/* FP:mod.rs-0316 */         let mut visited = DenseBitSet::new_empty(graph.len_nodes());
/* FP:mod.rs-0317 */         visited.insert(start_node.node_id());
/* FP:mod.rs-0318 */         DepthFirstTraversal { graph, stack: vec![start_node], visited, direction }
/* FP:mod.rs-0319 */     }
/* FP:mod.rs-0320 */ 
/* FP:mod.rs-0321 */     fn visit(&mut self, node: NodeIndex) {
/* FP:mod.rs-0322 */         if self.visited.insert(node.node_id()) {
/* FP:mod.rs-0323 */             self.stack.push(node);
/* FP:mod.rs-0324 */         }
/* FP:mod.rs-0325 */     }
/* FP:mod.rs-0326 */ }
/* FP:mod.rs-0327 */ 
/* FP:mod.rs-0328 */ impl<'g, N: Debug, E: Debug> Iterator for DepthFirstTraversal<'g, N, E> {
/* FP:mod.rs-0329 */     type Item = NodeIndex;
/* FP:mod.rs-0330 */ 
/* FP:mod.rs-0331 */     fn next(&mut self) -> Option<NodeIndex> {
/* FP:mod.rs-0332 */         let next = self.stack.pop();
/* FP:mod.rs-0333 */         if let Some(idx) = next {
/* FP:mod.rs-0334 */             for (_, edge) in self.graph.adjacent_edges(idx, self.direction) {
/* FP:mod.rs-0335 */                 let target = edge.source_or_target(self.direction);
/* FP:mod.rs-0336 */                 self.visit(target);
/* FP:mod.rs-0337 */             }
/* FP:mod.rs-0338 */         }
/* FP:mod.rs-0339 */         next
/* FP:mod.rs-0340 */     }
/* FP:mod.rs-0341 */ 
/* FP:mod.rs-0342 */     fn size_hint(&self) -> (usize, Option<usize>) {
/* FP:mod.rs-0343 */         // We will visit every node in the graph exactly once.
/* FP:mod.rs-0344 */         let remaining = self.graph.len_nodes() - self.visited.count();
/* FP:mod.rs-0345 */         (remaining, Some(remaining))
/* FP:mod.rs-0346 */     }
/* FP:mod.rs-0347 */ }
/* FP:mod.rs-0348 */ 
/* FP:mod.rs-0349 */ impl<'g, N: Debug, E: Debug> ExactSizeIterator for DepthFirstTraversal<'g, N, E> {}
/* FP:mod.rs-0350 */ 
/* FP:mod.rs-0351 */ impl<E> Edge<E> {
/* FP:mod.rs-0352 */     pub fn source(&self) -> NodeIndex {
/* FP:mod.rs-0353 */         self.source
/* FP:mod.rs-0354 */     }
/* FP:mod.rs-0355 */ 
/* FP:mod.rs-0356 */     pub fn target(&self) -> NodeIndex {
/* FP:mod.rs-0357 */         self.target
/* FP:mod.rs-0358 */     }
/* FP:mod.rs-0359 */ 
/* FP:mod.rs-0360 */     pub fn source_or_target(&self, direction: Direction) -> NodeIndex {
/* FP:mod.rs-0361 */         if direction == OUTGOING { self.target } else { self.source }
/* FP:mod.rs-0362 */     }
/* FP:mod.rs-0363 */ }