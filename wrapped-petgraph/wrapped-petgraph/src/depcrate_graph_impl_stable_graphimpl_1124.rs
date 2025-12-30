// Generated macro for impl_1124 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1124 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1124"}
// Dependencies: {}
impl < 'a , Ix , E > EdgeReference < 'a , E , Ix > where Ix : IndexType , { # [doc = " Access the edge’s weight."] # [doc = ""] # [doc = " **NOTE** that this method offers a longer lifetime"] # [doc = " than the trait (unfortunately they don't match yet)."] pub fn weight (& self) -> & 'a E { self . weight } }
};
}
