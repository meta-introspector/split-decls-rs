// Generated macro for impl_1350 (impl)
macro_rules! Depcrate_matrix_graphimpl_1350 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1350"}
// Dependencies: {}
impl < 'a , N , E , S , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > IntoEdgeReferences for & 'a MatrixGraph < N , E , S , Ty , Null , Ix > { type EdgeRef = (NodeIndex < Ix > , NodeIndex < Ix > , & 'a E) ; type EdgeReferences = EdgeReferences < 'a , Ty , Null , Ix > ; fn edge_references (self) -> Self :: EdgeReferences { EdgeReferences :: new (& self . node_adjacencies , self . node_capacity) } }
};
}
