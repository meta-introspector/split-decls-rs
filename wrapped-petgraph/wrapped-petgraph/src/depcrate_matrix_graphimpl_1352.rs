// Generated macro for impl_1352 (impl)
macro_rules! Depcrate_matrix_graphimpl_1352 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1352"}
// Dependencies: {}
impl < 'a , N , E , S : BuildHasher , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoEdgesDirected for & 'a MatrixGraph < N , E , S , Directed , Null , Ix > { type EdgesDirected = Edges < 'a , Directed , Null , Ix > ; fn edges_directed (self , a : Self :: NodeId , dir : Direction) -> Self :: EdgesDirected { MatrixGraph :: edges_directed (self , a , dir) } }
};
}
