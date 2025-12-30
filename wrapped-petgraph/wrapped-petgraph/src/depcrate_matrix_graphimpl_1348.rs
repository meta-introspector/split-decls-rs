// Generated macro for impl_1348 (impl)
macro_rules! Depcrate_matrix_graphimpl_1348 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1348"}
// Dependencies: {}
impl < 'a , N , E : 'a , S : BuildHasher , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoNeighborsDirected for & 'a MatrixGraph < N , E , S , Directed , Null , Ix > { type NeighborsDirected = Neighbors < 'a , Directed , Null , Ix > ; fn neighbors_directed (self , a : NodeIndex < Ix > , d : Direction) -> Self :: NeighborsDirected { MatrixGraph :: neighbors_directed (self , a , d) } }
};
}
