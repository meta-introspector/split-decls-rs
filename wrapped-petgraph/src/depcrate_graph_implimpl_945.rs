// Generated macro for impl_945 (impl)
macro_rules! Depcrate_graph_implimpl_945 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_945"}
// Dependencies: {}
impl fmt :: Display for GraphError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { GraphError :: NodeIxLimit => write ! (f , "The Graph is at the maximum number of nodes for its index") , GraphError :: EdgeIxLimit => write ! (f , "The Graph is at the maximum number of edges for its index.") , GraphError :: NodeMissed (i) => { write ! (f , "The node with index {i} is missing from the graph.") } GraphError :: NodeOutBounds => write ! (f , "Node indices out of bounds.") , } } }
};
}
