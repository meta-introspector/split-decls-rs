// Generated macro for impl_1353 (impl)
macro_rules! Depcrate_matrix_graphimpl_1353 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1353"}
// Dependencies: {}
impl < N , E , S , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > NodeIndexable for MatrixGraph < N , E , S , Ty , Null , Ix > { fn node_bound (& self) -> usize { self . nodes . upper_bound } fn to_index (& self , ix : NodeIndex < Ix >) -> usize { ix . index () } fn from_index (& self , ix : usize) -> Self :: NodeId { NodeIndex :: new (ix) } }
};
}
