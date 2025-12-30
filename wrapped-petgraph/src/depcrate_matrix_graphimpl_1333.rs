// Generated macro for impl_1333 (impl)
macro_rules! Depcrate_matrix_graphimpl_1333 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1333"}
// Dependencies: {}
impl < N , E , S : BuildHasher + Default > MatrixGraph < N , E , S , Directed > { # [doc = " Create a new `MatrixGraph` with directed edges."] # [doc = ""] # [doc = " This is a convenience method. Use `MatrixGraph::with_capacity` or `MatrixGraph::default` for"] # [doc = " a constructor that is generic in all the type parameters of `MatrixGraph`."] pub fn new () -> Self { MatrixGraph :: default () } }
};
}
