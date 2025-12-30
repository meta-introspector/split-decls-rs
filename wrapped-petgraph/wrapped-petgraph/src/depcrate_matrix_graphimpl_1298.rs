// Generated macro for impl_1298 (impl)
macro_rules! Depcrate_matrix_graphimpl_1298 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1298"}
// Dependencies: {}
impl fmt :: Display for MatrixError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MatrixError :: NodeIxLimit => write ! (f , "The MatrixGraph is at the maximum number of nodes for its index") , MatrixError :: NodeMissed (i) => { write ! (f , "The node with index {i} is missing from the graph.") } } } }
};
}
