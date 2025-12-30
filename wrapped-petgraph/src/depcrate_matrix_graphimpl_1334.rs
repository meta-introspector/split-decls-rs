// Generated macro for impl_1334 (impl)
macro_rules! Depcrate_matrix_graphimpl_1334 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1334"}
// Dependencies: {}
impl < N , E , S : BuildHasher + Default > MatrixGraph < N , E , S , Undirected > { # [doc = " Create a new `MatrixGraph` with undirected edges."] # [doc = ""] # [doc = " This is a convenience method. Use `MatrixGraph::with_capacity` or `MatrixGraph::default` for"] # [doc = " a constructor that is generic in all the type parameters of `MatrixGraph`."] pub fn new_undirected () -> Self { MatrixGraph :: default () } }
};
}
