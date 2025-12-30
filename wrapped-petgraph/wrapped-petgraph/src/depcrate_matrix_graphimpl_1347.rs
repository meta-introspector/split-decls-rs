// Generated macro for impl_1347 (impl)
macro_rules! Depcrate_matrix_graphimpl_1347 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1347"}
// Dependencies: {}
impl < 'a , N , E : 'a , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoNeighbors for & 'a MatrixGraph < N , E , S , Ty , Null , Ix > { type Neighbors = Neighbors < 'a , Ty , Null , Ix > ; fn neighbors (self , a : NodeIndex < Ix >) -> Self :: Neighbors { MatrixGraph :: neighbors (self , a) } }
};
}
