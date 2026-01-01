/* FP:mod.rs-0001 */ use crate::rustc_index::Idx;
/* FP:mod.rs-0002 */ 
/* FP:mod.rs-0010 */ 
/* FP:mod.rs-0011 */ #[cfg(test)]
/* FP:mod.rs-0013 */ 
/* FP:mod.rs-0014 */ pub trait DirectedGraph {
/* FP:mod.rs-0015 */     type Node: Idx;
/* FP:mod.rs-0016 */ 
/* FP:mod.rs-0017 */     /// Returns the total number of nodes in this graph.
/* FP:mod.rs-0018 */     ///
/* FP:mod.rs-0019 */     /// Several graph algorithm implementations assume that every node ID is
/* FP:mod.rs-0020 */     /// strictly less than the number of nodes, i.e. nodes are densely numbered.
/* FP:mod.rs-0021 */     /// That assumption allows them to use `num_nodes` to allocate per-node
/* FP:mod.rs-0022 */     /// data structures, indexed by node.
/* FP:mod.rs-0023 */     fn num_nodes(&self) -> usize;
/* FP:mod.rs-0024 */ 
/* FP:mod.rs-0025 */     /// Iterates over all nodes of a graph in ascending numeric order.
/* FP:mod.rs-0026 */     ///
/* FP:mod.rs-0027 */     /// Assumes that nodes are densely numbered, i.e. every index in
/* FP:mod.rs-0028 */     /// `0..num_nodes` is a valid node.
/* FP:mod.rs-0029 */     fn iter_nodes(
/* FP:mod.rs-0030 */         &self,
/* FP:mod.rs-0031 */     ) -> impl Iterator<Item = Self::Node> + DoubleEndedIterator + ExactSizeIterator {
/* FP:mod.rs-0032 */         (0..self.num_nodes()).map(<Self::Node as Idx>::new)
/* FP:mod.rs-0033 */     }
/* FP:mod.rs-0034 */ }
/* FP:mod.rs-0035 */ 
/* FP:mod.rs-0036 */ pub trait NumEdges: DirectedGraph {
/* FP:mod.rs-0037 */     fn num_edges(&self) -> usize;
/* FP:mod.rs-0038 */ }
/* FP:mod.rs-0039 */ 
/* FP:mod.rs-0040 */ pub trait StartNode: DirectedGraph {
/* FP:mod.rs-0041 */     fn start_node(&self) -> Self::Node;
/* FP:mod.rs-0042 */ }
/* FP:mod.rs-0043 */ 
/* FP:mod.rs-0044 */ pub trait Successors: DirectedGraph {
/* FP:mod.rs-0045 */     fn successors(&self, node: Self::Node) -> impl Iterator<Item = Self::Node>;
/* FP:mod.rs-0046 */ }
/* FP:mod.rs-0047 */ 
/* FP:mod.rs-0048 */ pub trait Predecessors: DirectedGraph {
/* FP:mod.rs-0049 */     fn predecessors(&self, node: Self::Node) -> impl Iterator<Item = Self::Node>;
/* FP:mod.rs-0050 */ }
/* FP:mod.rs-0051 */ 
/* FP:mod.rs-0052 */ /// Alias for [`DirectedGraph`] + [`StartNode`] + [`Predecessors`] + [`Successors`].
/* FP:mod.rs-0053 */ pub trait ControlFlowGraph: DirectedGraph + StartNode + Predecessors + Successors {}
/* FP:mod.rs-0054 */ impl<T> ControlFlowGraph for T where T: DirectedGraph + StartNode + Predecessors + Successors {}
/* FP:mod.rs-0055 */ 
/* FP:mod.rs-0056 */ /// Returns `true` if the graph has a cycle that is reachable from the start node.
/* FP:mod.rs-0057 */ pub fn is_cyclic<G>(graph: &G) -> bool
/* FP:mod.rs-0058 */ where
/* FP:mod.rs-0059 */     G: ?Sized + DirectedGraph + StartNode + Successors,
/* FP:mod.rs-0060 */ {
/* FP:mod.rs-0061 */     iterate::TriColorDepthFirstSearch::new(graph)
/* FP:mod.rs-0062 */         .run_from_start(&mut iterate::CycleDetector)
/* FP:mod.rs-0063 */         .is_some()
/* FP:mod.rs-0064 */ }
/* FP:mod.rs-0065 */ 
/* FP:mod.rs-0066 */ pub fn depth_first_search<G>(graph: G, from: G::Node) -> iterate::DepthFirstSearch<G>
/* FP:mod.rs-0067 */ where
/* FP:mod.rs-0068 */     G: Successors,
/* FP:mod.rs-0069 */ {
/* FP:mod.rs-0070 */     iterate::DepthFirstSearch::new(graph).with_start_node(from)
/* FP:mod.rs-0071 */ }
/* FP:mod.rs-0072 */ 
/* FP:mod.rs-0073 */ pub fn depth_first_search_as_undirected<G>(
/* FP:mod.rs-0074 */     graph: G,
/* FP:mod.rs-0075 */     from: G::Node,
/* FP:mod.rs-0076 */ ) -> iterate::DepthFirstSearch<impl Successors<Node = G::Node>>
/* FP:mod.rs-0077 */ where
/* FP:mod.rs-0078 */     G: Successors + Predecessors,
/* FP:mod.rs-0079 */ {
/* FP:mod.rs-0080 */     struct AsUndirected<G>(G);
/* FP:mod.rs-0081 */ 
/* FP:mod.rs-0082 */     impl<G: DirectedGraph> DirectedGraph for AsUndirected<G> {
/* FP:mod.rs-0083 */         type Node = G::Node;
/* FP:mod.rs-0084 */ 
/* FP:mod.rs-0085 */         fn num_nodes(&self) -> usize {
/* FP:mod.rs-0086 */             self.0.num_nodes()
/* FP:mod.rs-0087 */         }
/* FP:mod.rs-0088 */     }
/* FP:mod.rs-0089 */ 
/* FP:mod.rs-0090 */     impl<G: Successors + Predecessors> Successors for AsUndirected<G> {
/* FP:mod.rs-0091 */         fn successors(&self, node: Self::Node) -> impl Iterator<Item = Self::Node> {
/* FP:mod.rs-0092 */             self.0.successors(node).chain(self.0.predecessors(node))
/* FP:mod.rs-0093 */         }
/* FP:mod.rs-0094 */     }
/* FP:mod.rs-0095 */ 
/* FP:mod.rs-0096 */     iterate::DepthFirstSearch::new(AsUndirected(graph)).with_start_node(from)
/* FP:mod.rs-0097 */ }