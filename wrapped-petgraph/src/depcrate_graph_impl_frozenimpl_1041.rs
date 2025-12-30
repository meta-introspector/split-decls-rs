// Generated macro for impl_1041 (impl)
macro_rules! Depcrate_graph_impl_frozenimpl_1041 {
() => {
// Module: crate::graph_impl::frozen
// Provides: {"impl_1041"}
// Dependencies: {}
impl < G , I > IndexMut < I > for Frozen < '_ , G > where G : IndexMut < I > , { fn index_mut (& mut self , i : I) -> & mut G :: Output { self . 0 . index_mut (i) } }
};
}
