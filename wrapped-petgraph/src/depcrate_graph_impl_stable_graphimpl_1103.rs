// Generated macro for impl_1103 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1103 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1103"}
// Dependencies: {}
impl < N , Ix > Clone for StableGraphNode < N , Ix > where N : Clone , Ix : Copy , { fn clone (& self) -> Self { Self { index : self . index , weight : self . weight . clone () , } } }
};
}
