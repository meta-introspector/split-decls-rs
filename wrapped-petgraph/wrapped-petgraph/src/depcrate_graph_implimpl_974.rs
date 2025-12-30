// Generated macro for impl_974 (impl)
macro_rules! Depcrate_graph_implimpl_974 {
() => {
// Module: crate::graph_impl
// Provides: {"impl_974"}
// Dependencies: {}
impl < E , Ty , Ix > Clone for Edges < '_ , E , Ty , Ix > where Ix : IndexType , Ty : EdgeType , { fn clone (& self) -> Self { Edges { skip_start : self . skip_start , edges : self . edges , next : self . next , direction : self . direction , ty : self . ty , } } }
};
}
