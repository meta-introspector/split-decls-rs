// Generated macro for impl_1022 (impl)
macro_rules! Depcrate_graph_implimpl_1022 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1022"}
// Dependencies: {}
impl < 'a , Ix , E > EdgeReference < 'a , E , Ix > where Ix : IndexType , { # [doc = " Access the edge’s weight."] # [doc = ""] # [doc = " **NOTE** that this method offers a longer lifetime"] # [doc = " than the trait (unfortunately they don't match yet)."] pub fn weight (& self) -> & 'a E { self . weight } }
};
}
