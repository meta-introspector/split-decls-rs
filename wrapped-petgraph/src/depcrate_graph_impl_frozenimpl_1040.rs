// Generated macro for impl_1040 (impl)
macro_rules! Depcrate_graph_impl_frozenimpl_1040 {
() => {
// Module: crate::graph_impl::frozen
// Provides: {"impl_1040"}
// Dependencies: {}
impl < G , I > Index < I > for Frozen < '_ , G > where G : Index < I > , { type Output = G :: Output ; fn index (& self , i : I) -> & G :: Output { self . 0 . index (i) } }
};
}
