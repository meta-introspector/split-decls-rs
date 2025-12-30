// Generated macro for impl_1148 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1148 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1148"}
// Dependencies: {}
impl < N , E , Ty , Ix > visit :: Visitable for StableGraph < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Map = FixedBitSet ; fn visit_map (& self) -> FixedBitSet { FixedBitSet :: with_capacity (self . node_bound ()) } fn reset_map (& self , map : & mut Self :: Map) { map . clear () ; map . grow (self . node_bound ()) ; } }
};
}
