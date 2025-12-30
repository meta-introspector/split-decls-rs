// Generated macro for impl_1105 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1105 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1105"}
// Dependencies: {}
impl < E , Ix > Clone for StableGraphEdge < E , Ix > where E : Clone , Ix : Copy , { fn clone (& self) -> Self { Self { index : self . index , source : self . source , target : self . target , weight : self . weight . clone () , } } }
};
}
