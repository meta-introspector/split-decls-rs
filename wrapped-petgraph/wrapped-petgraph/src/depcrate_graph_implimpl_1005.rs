// Generated macro for impl_1005 (impl)
macro_rules! Depcrate_graph_implimpl_1005 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_1005"}
// Dependencies: {}
impl < E , Ix : IndexType > PartialEq for EdgeReference < '_ , E , Ix > where E : PartialEq , { fn eq (& self , rhs : & Self) -> bool { self . index == rhs . index && self . weight == rhs . weight } }
};
}
