/* FP:mod.rs-0001 */ use std::ops::ControlFlow;
/* FP:mod.rs-0002 */ 
/* FP:mod.rs-0003 */ use crate::rustc_index::bit_set::DenseBitSet;
/* FP:mod.rs-0004 */ use crate::rustc_index::{IndexSlice, IndexVec};
/* FP:mod.rs-0005 */ 
/* FP:mod.rs-0006 */ use super::{DirectedGraph, StartNode, Successors};
/* FP:mod.rs-0007 */ 
/* FP:mod.rs-0008 */ #[cfg(test)]
/* FP:mod.rs-0010 */ 
/* FP:mod.rs-0011 */ pub fn post_order_from<G: DirectedGraph + Successors>(
/* FP:mod.rs-0012 */     graph: &G,
/* FP:mod.rs-0013 */     start_node: G::Node,
/* FP:mod.rs-0014 */ ) -> Vec<G::Node> {
/* FP:mod.rs-0015 */     post_order_from_to(graph, start_node, None)
/* FP:mod.rs-0016 */ }
/* FP:mod.rs-0017 */ 
/* FP:mod.rs-0018 */ pub fn post_order_from_to<G: DirectedGraph + Successors>(
/* FP:mod.rs-0019 */     graph: &G,
/* FP:mod.rs-0020 */     start_node: G::Node,
/* FP:mod.rs-0021 */     end_node: Option<G::Node>,
/* FP:mod.rs-0022 */ ) -> Vec<G::Node> {
/* FP:mod.rs-0023 */     let mut visited: IndexVec<G::Node, bool> = IndexVec::from_elem_n(false, graph.num_nodes());
/* FP:mod.rs-0024 */     let mut result: Vec<G::Node> = Vec::with_capacity(graph.num_nodes());
/* FP:mod.rs-0025 */     if let Some(end_node) = end_node {
/* FP:mod.rs-0026 */         visited[end_node] = true;
/* FP:mod.rs-0027 */     }
/* FP:mod.rs-0028 */     post_order_walk(graph, start_node, &mut result, &mut visited);
/* FP:mod.rs-0029 */     result
/* FP:mod.rs-0030 */ }
/* FP:mod.rs-0031 */ 
/* FP:mod.rs-0032 */ fn post_order_walk<G: DirectedGraph + Successors>(
/* FP:mod.rs-0033 */     graph: &G,
/* FP:mod.rs-0034 */     node: G::Node,
/* FP:mod.rs-0035 */     result: &mut Vec<G::Node>,
/* FP:mod.rs-0036 */     visited: &mut IndexSlice<G::Node, bool>,
/* FP:mod.rs-0037 */ ) {
/* FP:mod.rs-0038 */     struct PostOrderFrame<Node, Iter> {
/* FP:mod.rs-0039 */         node: Node,
/* FP:mod.rs-0040 */         iter: Iter,
/* FP:mod.rs-0041 */     }
/* FP:mod.rs-0042 */ 
/* FP:mod.rs-0043 */     if visited[node] {
/* FP:mod.rs-0044 */         return;
/* FP:mod.rs-0045 */     }
/* FP:mod.rs-0046 */ 
/* FP:mod.rs-0047 */     let mut stack = vec![PostOrderFrame { node, iter: graph.successors(node) }];
/* FP:mod.rs-0048 */ 
/* FP:mod.rs-0049 */     'recurse: while let Some(frame) = stack.last_mut() {
/* FP:mod.rs-0050 */         let node = frame.node;
/* FP:mod.rs-0051 */         visited[node] = true;
/* FP:mod.rs-0052 */ 
/* FP:mod.rs-0053 */         for successor in frame.iter.by_ref() {
/* FP:mod.rs-0054 */             if !visited[successor] {
/* FP:mod.rs-0055 */                 stack.push(PostOrderFrame { node: successor, iter: graph.successors(successor) });
/* FP:mod.rs-0056 */                 continue 'recurse;
/* FP:mod.rs-0057 */             }
/* FP:mod.rs-0058 */         }
/* FP:mod.rs-0059 */ 
/* FP:mod.rs-0060 */         let _ = stack.pop();
/* FP:mod.rs-0061 */         result.push(node);
/* FP:mod.rs-0062 */     }
/* FP:mod.rs-0063 */ }
/* FP:mod.rs-0064 */ 
/* FP:mod.rs-0065 */ pub fn reverse_post_order<G: DirectedGraph + Successors>(
/* FP:mod.rs-0066 */     graph: &G,
/* FP:mod.rs-0067 */     start_node: G::Node,
/* FP:mod.rs-0068 */ ) -> Vec<G::Node> {
/* FP:mod.rs-0069 */     let mut vec = post_order_from(graph, start_node);
/* FP:mod.rs-0070 */     vec.reverse();
/* FP:mod.rs-0071 */     vec
/* FP:mod.rs-0072 */ }
/* FP:mod.rs-0073 */ 
/* FP:mod.rs-0074 */ /// A "depth-first search" iterator for a directed graph.
/* FP:mod.rs-0075 */ pub struct DepthFirstSearch<G>
/* FP:mod.rs-0076 */ where
/* FP:mod.rs-0077 */     G: DirectedGraph + Successors,
/* FP:mod.rs-0078 */ {
/* FP:mod.rs-0079 */     graph: G,
/* FP:mod.rs-0080 */     stack: Vec<G::Node>,
/* FP:mod.rs-0081 */     visited: DenseBitSet<G::Node>,
/* FP:mod.rs-0082 */ }
/* FP:mod.rs-0083 */ 
/* FP:mod.rs-0084 */ impl<G> DepthFirstSearch<G>
/* FP:mod.rs-0085 */ where
/* FP:mod.rs-0086 */     G: DirectedGraph + Successors,
/* FP:mod.rs-0087 */ {
/* FP:mod.rs-0088 */     pub fn new(graph: G) -> Self {
/* FP:mod.rs-0089 */         Self { stack: vec![], visited: DenseBitSet::new_empty(graph.num_nodes()), graph }
/* FP:mod.rs-0090 */     }
/* FP:mod.rs-0091 */ 
/* FP:mod.rs-0092 */     /// Version of `push_start_node` that is convenient for chained
/* FP:mod.rs-0093 */     /// use.
/* FP:mod.rs-0094 */     pub fn with_start_node(mut self, start_node: G::Node) -> Self {
/* FP:mod.rs-0095 */         self.push_start_node(start_node);
/* FP:mod.rs-0096 */         self
/* FP:mod.rs-0097 */     }
/* FP:mod.rs-0098 */ 
/* FP:mod.rs-0099 */     /// Pushes another start node onto the stack. If the node
/* FP:mod.rs-0100 */     /// has not already been visited, then you will be able to
/* FP:mod.rs-0101 */     /// walk its successors (and so forth) after the current
/* FP:mod.rs-0102 */     /// contents of the stack are drained. If multiple start nodes
/* FP:mod.rs-0103 */     /// are added into the walk, then their mutual successors
/* FP:mod.rs-0104 */     /// will all be walked. You can use this method once the
/* FP:mod.rs-0105 */     /// iterator has been completely drained to add additional
/* FP:mod.rs-0106 */     /// start nodes.
/* FP:mod.rs-0107 */     pub fn push_start_node(&mut self, start_node: G::Node) {
/* FP:mod.rs-0108 */         if self.visited.insert(start_node) {
/* FP:mod.rs-0109 */             self.stack.push(start_node);
/* FP:mod.rs-0110 */         }
/* FP:mod.rs-0111 */     }
/* FP:mod.rs-0112 */ 
/* FP:mod.rs-0113 */     /// Searches all nodes reachable from the current start nodes.
/* FP:mod.rs-0114 */     /// This is equivalent to just invoke `next` repeatedly until
/* FP:mod.rs-0115 */     /// you get a `None` result.
/* FP:mod.rs-0116 */     pub fn complete_search(&mut self) {
/* FP:mod.rs-0117 */         for _ in self.by_ref() {}
/* FP:mod.rs-0118 */     }
/* FP:mod.rs-0119 */ 
/* FP:mod.rs-0120 */     /// Returns true if node has been visited thus far.
/* FP:mod.rs-0121 */     /// A node is considered "visited" once it is pushed
/* FP:mod.rs-0122 */     /// onto the internal stack; it may not yet have been yielded
/* FP:mod.rs-0123 */     /// from the iterator. This method is best used after
/* FP:mod.rs-0124 */     /// the iterator is completely drained.
/* FP:mod.rs-0125 */     pub fn visited(&self, node: G::Node) -> bool {
/* FP:mod.rs-0126 */         self.visited.contains(node)
/* FP:mod.rs-0127 */     }
/* FP:mod.rs-0128 */ 
/* FP:mod.rs-0129 */     /// Returns a reference to the set of nodes that have been visited, with
/* FP:mod.rs-0130 */     /// the same caveats as [`Self::visited`].
/* FP:mod.rs-0131 */     ///
/* FP:mod.rs-0132 */     /// When incorporating the visited nodes into another bitset, using bulk
/* FP:mod.rs-0133 */     /// operations like `union` or `intersect` can be more efficient than
/* FP:mod.rs-0134 */     /// processing each node individually.
/* FP:mod.rs-0135 */     pub fn visited_set(&self) -> &DenseBitSet<G::Node> {
/* FP:mod.rs-0136 */         &self.visited
/* FP:mod.rs-0137 */     }
/* FP:mod.rs-0138 */ }
/* FP:mod.rs-0139 */ 
/* FP:mod.rs-0140 */ impl<G> std::fmt::Debug for DepthFirstSearch<G>
/* FP:mod.rs-0141 */ where
/* FP:mod.rs-0142 */     G: DirectedGraph + Successors,
/* FP:mod.rs-0143 */ {
/* FP:mod.rs-0144 */     fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
/* FP:mod.rs-0145 */         let mut f = fmt.debug_set();
/* FP:mod.rs-0146 */         for n in self.visited.iter() {
/* FP:mod.rs-0147 */             f.entry(&n);
/* FP:mod.rs-0148 */         }
/* FP:mod.rs-0149 */         f.finish()
/* FP:mod.rs-0150 */     }
/* FP:mod.rs-0151 */ }
/* FP:mod.rs-0152 */ 
/* FP:mod.rs-0153 */ impl<G> Iterator for DepthFirstSearch<G>
/* FP:mod.rs-0154 */ where
/* FP:mod.rs-0155 */     G: DirectedGraph + Successors,
/* FP:mod.rs-0156 */ {
/* FP:mod.rs-0157 */     type Item = G::Node;
/* FP:mod.rs-0158 */ 
/* FP:mod.rs-0159 */     fn next(&mut self) -> Option<G::Node> {
/* FP:mod.rs-0160 */         let DepthFirstSearch { stack, visited, graph } = self;
/* FP:mod.rs-0161 */         let n = stack.pop()?;
/* FP:mod.rs-0162 */         stack.extend(graph.successors(n).filter(|&m| visited.insert(m)));
/* FP:mod.rs-0163 */         Some(n)
/* FP:mod.rs-0164 */     }
/* FP:mod.rs-0165 */ }
/* FP:mod.rs-0166 */ 
/* FP:mod.rs-0167 */ /// The status of a node in the depth-first search.
/* FP:mod.rs-0168 */ ///
/* FP:mod.rs-0169 */ /// See the documentation of `TriColorDepthFirstSearch` to see how a node's status is updated
/* FP:mod.rs-0170 */ /// during DFS.
/* FP:mod.rs-0171 */ #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/* FP:mod.rs-0172 */ pub enum NodeStatus {
/* FP:mod.rs-0173 */     /// This node has been examined by the depth-first search but is not yet `Settled`.
/* FP:mod.rs-0174 */     ///
/* FP:mod.rs-0175 */     /// Also referred to as "gray" or "discovered" nodes in [CLR].
/* FP:mod.rs-0176 */     ///
/* FP:mod.rs-0177 */     /// [CLR]: https://en.wikipedia.org/wiki/Introduction_to_Algorithms
/* FP:mod.rs-0178 */     Visited,
/* FP:mod.rs-0179 */ 
/* FP:mod.rs-0180 */     /// This node and all nodes reachable from it have been examined by the depth-first search.
/* FP:mod.rs-0181 */     ///
/* FP:mod.rs-0182 */     /// Also referred to as "black" or "finished" nodes in [CLR].
/* FP:mod.rs-0183 */     ///
/* FP:mod.rs-0184 */     /// [CLR]: https://en.wikipedia.org/wiki/Introduction_to_Algorithms
/* FP:mod.rs-0185 */     Settled,
/* FP:mod.rs-0186 */ }
/* FP:mod.rs-0187 */ 
/* FP:mod.rs-0188 */ struct Event<N> {
/* FP:mod.rs-0189 */     node: N,
/* FP:mod.rs-0190 */     becomes: NodeStatus,
/* FP:mod.rs-0191 */ }
/* FP:mod.rs-0192 */ 
/* FP:mod.rs-0193 */ /// A depth-first search that also tracks when all successors of a node have been examined.
/* FP:mod.rs-0194 */ ///
/* FP:mod.rs-0195 */ /// This is based on the DFS described in [Introduction to Algorithms (1st ed.)][CLR], hereby
/* FP:mod.rs-0196 */ /// referred to as **CLR**. However, we use the terminology in [`NodeStatus`] above instead of
/* FP:mod.rs-0197 */ /// "discovered"/"finished" or "white"/"grey"/"black". Each node begins the search with no status,
/* FP:mod.rs-0198 */ /// becomes `Visited` when it is first examined by the DFS and is `Settled` when all nodes
/* FP:mod.rs-0199 */ /// reachable from it have been examined. This allows us to differentiate between "tree", "back"
/* FP:mod.rs-0200 */ /// and "forward" edges (see [`TriColorVisitor::node_examined`]).
/* FP:mod.rs-0201 */ ///
/* FP:mod.rs-0202 */ /// Unlike the pseudocode in [CLR], this implementation is iterative and does not use timestamps.
/* FP:mod.rs-0203 */ /// We accomplish this by storing `Event`s on the stack that result in a (possible) state change
/* FP:mod.rs-0204 */ /// for each node. A `Visited` event signifies that we should examine this node if it has not yet
/* FP:mod.rs-0205 */ /// been `Visited` or `Settled`. When a node is examined for the first time, we mark it as
/* FP:mod.rs-0206 */ /// `Visited` and push a `Settled` event for it on stack followed by `Visited` events for all of
/* FP:mod.rs-0207 */ /// its predecessors, scheduling them for examination. Multiple `Visited` events for a single node
/* FP:mod.rs-0208 */ /// may exist on the stack simultaneously if a node has multiple predecessors, but only one
/* FP:mod.rs-0209 */ /// `Settled` event will ever be created for each node. After all `Visited` events for a node's
/* FP:mod.rs-0210 */ /// successors have been popped off the stack (as well as any new events triggered by visiting
/* FP:mod.rs-0211 */ /// those successors), we will pop off that node's `Settled` event.
/* FP:mod.rs-0212 */ ///
/* FP:mod.rs-0213 */ /// [CLR]: https://en.wikipedia.org/wiki/Introduction_to_Algorithms
/* FP:mod.rs-0214 */ pub struct TriColorDepthFirstSearch<'graph, G>
/* FP:mod.rs-0215 */ where
/* FP:mod.rs-0216 */     G: ?Sized + DirectedGraph + Successors,
/* FP:mod.rs-0217 */ {
/* FP:mod.rs-0218 */     graph: &'graph G,
/* FP:mod.rs-0219 */     stack: Vec<Event<G::Node>>,
/* FP:mod.rs-0220 */     visited: DenseBitSet<G::Node>,
/* FP:mod.rs-0221 */     settled: DenseBitSet<G::Node>,
/* FP:mod.rs-0222 */ }
/* FP:mod.rs-0223 */ 
/* FP:mod.rs-0224 */ impl<'graph, G> TriColorDepthFirstSearch<'graph, G>
/* FP:mod.rs-0225 */ where
/* FP:mod.rs-0226 */     G: ?Sized + DirectedGraph + Successors,
/* FP:mod.rs-0227 */ {
/* FP:mod.rs-0228 */     pub fn new(graph: &'graph G) -> Self {
/* FP:mod.rs-0229 */         TriColorDepthFirstSearch {
/* FP:mod.rs-0230 */             graph,
/* FP:mod.rs-0231 */             stack: vec![],
/* FP:mod.rs-0232 */             visited: DenseBitSet::new_empty(graph.num_nodes()),
/* FP:mod.rs-0233 */             settled: DenseBitSet::new_empty(graph.num_nodes()),
/* FP:mod.rs-0234 */         }
/* FP:mod.rs-0235 */     }
/* FP:mod.rs-0236 */ 
/* FP:mod.rs-0237 */     /// Performs a depth-first search, starting from the given `root`.
/* FP:mod.rs-0238 */     ///
/* FP:mod.rs-0239 */     /// This won't visit nodes that are not reachable from `root`.
/* FP:mod.rs-0240 */     pub fn run_from<V>(mut self, root: G::Node, visitor: &mut V) -> Option<V::BreakVal>
/* FP:mod.rs-0241 */     where
/* FP:mod.rs-0242 */         V: TriColorVisitor<G>,
/* FP:mod.rs-0243 */     {
/* FP:mod.rs-0244 */         use NodeStatus::{Settled, Visited};
/* FP:mod.rs-0245 */ 
/* FP:mod.rs-0246 */         self.stack.push(Event { node: root, becomes: Visited });
/* FP:mod.rs-0247 */ 
/* FP:mod.rs-0248 */         loop {
/* FP:mod.rs-0249 */             match self.stack.pop()? {
/* FP:mod.rs-0250 */                 Event { node, becomes: Settled } => {
/* FP:mod.rs-0251 */                     let not_previously_settled = self.settled.insert(node);
/* FP:mod.rs-0252 */                     assert!(not_previously_settled, "A node should be settled exactly once");
/* FP:mod.rs-0253 */                     if let ControlFlow::Break(val) = visitor.node_settled(node) {
/* FP:mod.rs-0254 */                         return Some(val);
/* FP:mod.rs-0255 */                     }
/* FP:mod.rs-0256 */                 }
/* FP:mod.rs-0257 */ 
/* FP:mod.rs-0258 */                 Event { node, becomes: Visited } => {
/* FP:mod.rs-0259 */                     let not_previously_visited = self.visited.insert(node);
/* FP:mod.rs-0260 */                     let prior_status = if not_previously_visited {
/* FP:mod.rs-0261 */                         None
/* FP:mod.rs-0262 */                     } else if self.settled.contains(node) {
/* FP:mod.rs-0263 */                         Some(Settled)
/* FP:mod.rs-0264 */                     } else {
/* FP:mod.rs-0265 */                         Some(Visited)
/* FP:mod.rs-0266 */                     };
/* FP:mod.rs-0267 */ 
/* FP:mod.rs-0268 */                     if let ControlFlow::Break(val) = visitor.node_examined(node, prior_status) {
/* FP:mod.rs-0269 */                         return Some(val);
/* FP:mod.rs-0270 */                     }
/* FP:mod.rs-0271 */ 
/* FP:mod.rs-0272 */                     // If this node has already been examined, we are done.
/* FP:mod.rs-0273 */                     if prior_status.is_some() {
/* FP:mod.rs-0274 */                         continue;
/* FP:mod.rs-0275 */                     }
/* FP:mod.rs-0276 */ 
/* FP:mod.rs-0277 */                     // Otherwise, push a `Settled` event for this node onto the stack, then
/* FP:mod.rs-0278 */                     // schedule its successors for examination.
/* FP:mod.rs-0279 */                     self.stack.push(Event { node, becomes: Settled });
/* FP:mod.rs-0280 */                     for succ in self.graph.successors(node) {
/* FP:mod.rs-0281 */                         if !visitor.ignore_edge(node, succ) {
/* FP:mod.rs-0282 */                             self.stack.push(Event { node: succ, becomes: Visited });
/* FP:mod.rs-0283 */                         }
/* FP:mod.rs-0284 */                     }
/* FP:mod.rs-0285 */                 }
/* FP:mod.rs-0286 */             }
/* FP:mod.rs-0287 */         }
/* FP:mod.rs-0288 */     }
/* FP:mod.rs-0289 */ }
/* FP:mod.rs-0290 */ 
/* FP:mod.rs-0291 */ impl<G> TriColorDepthFirstSearch<'_, G>
/* FP:mod.rs-0292 */ where
/* FP:mod.rs-0293 */     G: ?Sized + DirectedGraph + Successors + StartNode,
/* FP:mod.rs-0294 */ {
/* FP:mod.rs-0295 */     /// Performs a depth-first search, starting from `G::start_node()`.
/* FP:mod.rs-0296 */     ///
/* FP:mod.rs-0297 */     /// This won't visit nodes that are not reachable from the start node.
/* FP:mod.rs-0298 */     pub fn run_from_start<V>(self, visitor: &mut V) -> Option<V::BreakVal>
/* FP:mod.rs-0299 */     where
/* FP:mod.rs-0300 */         V: TriColorVisitor<G>,
/* FP:mod.rs-0301 */     {
/* FP:mod.rs-0302 */         let root = self.graph.start_node();
/* FP:mod.rs-0303 */         self.run_from(root, visitor)
/* FP:mod.rs-0304 */     }
/* FP:mod.rs-0305 */ }
/* FP:mod.rs-0306 */ 
/* FP:mod.rs-0307 */ /// What to do when a node is examined or becomes `Settled` during DFS.
/* FP:mod.rs-0308 */ pub trait TriColorVisitor<G>
/* FP:mod.rs-0309 */ where
/* FP:mod.rs-0310 */     G: ?Sized + DirectedGraph,
/* FP:mod.rs-0311 */ {
/* FP:mod.rs-0312 */     /// The value returned by this search.
/* FP:mod.rs-0313 */     type BreakVal;
/* FP:mod.rs-0314 */ 
/* FP:mod.rs-0315 */     /// Called when a node is examined by the depth-first search.
/* FP:mod.rs-0316 */     ///
/* FP:mod.rs-0317 */     /// By checking the value of `prior_status`, this visitor can determine whether the edge
/* FP:mod.rs-0318 */     /// leading to this node was a tree edge (`None`), forward edge (`Some(Settled)`) or back edge
/* FP:mod.rs-0319 */     /// (`Some(Visited)`). For a full explanation of each edge type, see the "Depth-first Search"
/* FP:mod.rs-0320 */     /// chapter in [CLR] or [wikipedia].
/* FP:mod.rs-0321 */     ///
/* FP:mod.rs-0322 */     /// If you want to know *both* nodes linked by each edge, you'll need to modify
/* FP:mod.rs-0323 */     /// `TriColorDepthFirstSearch` to store a `source` node for each `Visited` event.
/* FP:mod.rs-0324 */     ///
/* FP:mod.rs-0325 */     /// [wikipedia]: https://en.wikipedia.org/wiki/Depth-first_search#Output_of_a_depth-first_search
/* FP:mod.rs-0326 */     /// [CLR]: https://en.wikipedia.org/wiki/Introduction_to_Algorithms
/* FP:mod.rs-0327 */     fn node_examined(
/* FP:mod.rs-0328 */         &mut self,
/* FP:mod.rs-0329 */         _node: G::Node,
/* FP:mod.rs-0330 */         _prior_status: Option<NodeStatus>,
/* FP:mod.rs-0331 */     ) -> ControlFlow<Self::BreakVal> {
/* FP:mod.rs-0332 */         ControlFlow::Continue(())
/* FP:mod.rs-0333 */     }
/* FP:mod.rs-0334 */ 
/* FP:mod.rs-0335 */     /// Called after all nodes reachable from this one have been examined.
/* FP:mod.rs-0336 */     fn node_settled(&mut self, _node: G::Node) -> ControlFlow<Self::BreakVal> {
/* FP:mod.rs-0337 */         ControlFlow::Continue(())
/* FP:mod.rs-0338 */     }
/* FP:mod.rs-0339 */ 
/* FP:mod.rs-0340 */     /// Behave as if no edges exist from `source` to `target`.
/* FP:mod.rs-0341 */     fn ignore_edge(&mut self, _source: G::Node, _target: G::Node) -> bool {
/* FP:mod.rs-0342 */         false
/* FP:mod.rs-0343 */     }
/* FP:mod.rs-0344 */ }
/* FP:mod.rs-0345 */ 
/* FP:mod.rs-0346 */ /// This `TriColorVisitor` looks for back edges in a graph, which indicate that a cycle exists.
/* FP:mod.rs-0347 */ pub struct CycleDetector;
/* FP:mod.rs-0348 */ 
/* FP:mod.rs-0349 */ impl<G> TriColorVisitor<G> for CycleDetector
/* FP:mod.rs-0350 */ where
/* FP:mod.rs-0351 */     G: ?Sized + DirectedGraph,
/* FP:mod.rs-0352 */ {
/* FP:mod.rs-0353 */     type BreakVal = ();
/* FP:mod.rs-0354 */ 
/* FP:mod.rs-0355 */     fn node_examined(
/* FP:mod.rs-0356 */         &mut self,
/* FP:mod.rs-0357 */         _node: G::Node,
/* FP:mod.rs-0358 */         prior_status: Option<NodeStatus>,
/* FP:mod.rs-0359 */     ) -> ControlFlow<Self::BreakVal> {
/* FP:mod.rs-0360 */         match prior_status {
/* FP:mod.rs-0361 */             Some(NodeStatus::Visited) => ControlFlow::Break(()),
/* FP:mod.rs-0362 */             _ => ControlFlow::Continue(()),
/* FP:mod.rs-0363 */         }
/* FP:mod.rs-0364 */     }
/* FP:mod.rs-0365 */ }