// Generated macro for impl_1343 (impl)
macro_rules! Depcrate_matrix_graphimpl_1343 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1343"}
// Dependencies: {}
impl < N , E , S , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > GraphBase for MatrixGraph < N , E , S , Ty , Null , Ix > { type NodeId = NodeIndex < Ix > ; type EdgeId = (NodeIndex < Ix > , NodeIndex < Ix >) ; }
};
}
