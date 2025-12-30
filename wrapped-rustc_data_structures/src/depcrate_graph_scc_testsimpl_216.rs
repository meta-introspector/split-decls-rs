// Generated macro for impl_216 (impl)
macro_rules! Depcrate_graph_scc_testsimpl_216 {
() => {
// Module: crate::graph::scc::tests
// Provides: {"impl_216"}
// Dependencies: {}
impl Annotation for MinMaxIn { fn merge_scc (self , other : Self) -> Self { Self { min : std :: cmp :: min (self . min , other . min) , max : std :: cmp :: max (self . max , other . max) } } fn merge_reached (self , _other : Self) -> Self { self } }
};
}
