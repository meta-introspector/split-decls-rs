// Generated macro for impl_1007 (impl)
macro_rules! Depcrate_graph_implimpl_1007 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1007"}
// Dependencies: {}
impl < N , E , Ty , Ix > visit :: Visitable for Graph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Map = FixedBitSet ; fn visit_map (& self) -> FixedBitSet { FixedBitSet :: with_capacity (self . node_count ()) } fn reset_map (& self , map : & mut Self :: Map) { map . clear () ; map . grow (self . node_count ()) ; } }
};
}
