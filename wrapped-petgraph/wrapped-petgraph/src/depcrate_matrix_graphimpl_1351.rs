// Generated macro for impl_1351 (impl)
macro_rules! Depcrate_matrix_graphimpl_1351 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1351"}
// Dependencies: {}
impl < 'a , N , E , S : BuildHasher , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoEdges for & 'a MatrixGraph < N , E , S , Ty , Null , Ix > { type Edges = Edges < 'a , Ty , Null , Ix > ; fn edges (self , a : Self :: NodeId) -> Self :: Edges { MatrixGraph :: edges (self , a) } }
};
}
