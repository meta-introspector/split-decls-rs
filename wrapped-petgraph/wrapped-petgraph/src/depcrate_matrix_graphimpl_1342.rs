// Generated macro for impl_1342 (impl)
macro_rules! Depcrate_matrix_graphimpl_1342 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1342"}
// Dependencies: {}
impl < N , E , S , Ty : EdgeType , Null : Nullable < Wrapped = E > , Ix : IndexType > Visitable for MatrixGraph < N , E , S , Ty , Null , Ix > { type Map = FixedBitSet ; fn visit_map (& self) -> FixedBitSet { FixedBitSet :: with_capacity (self . node_bound ()) } fn reset_map (& self , map : & mut Self :: Map) { map . clear () ; map . grow (self . node_bound ()) ; } }
};
}
