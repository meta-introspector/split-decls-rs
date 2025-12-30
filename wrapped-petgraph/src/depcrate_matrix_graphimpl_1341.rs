// Generated macro for impl_1341 (impl)
macro_rules! Depcrate_matrix_graphimpl_1341 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1341"}
// Dependencies: {}
impl < N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > GetAdjacencyMatrix for MatrixGraph < N , E , S , Ty , Null , Ix > { type AdjMatrix = () ; fn adjacency_matrix (& self) -> Self :: AdjMatrix { } fn is_adjacent (& self , _ : & Self :: AdjMatrix , a : NodeIndex < Ix > , b : NodeIndex < Ix >) -> bool { MatrixGraph :: has_edge (self , a , b) } }
};
}
