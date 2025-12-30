// Generated macro for impl_1123 (impl)
macro_rules! Depcrate_graph_impl_stable_graphimpl_1123 {
() => {
// Module: crate::graph_impl::stable_graph
// Provides: {"impl_1123"}
// Dependencies: {}
impl < E , Ix : IndexType > PartialEq for EdgeReference < '_ , E , Ix > where E : PartialEq , { fn eq (& self , rhs : & Self) -> bool { self . index == rhs . index && self . weight == rhs . weight } }
};
}
