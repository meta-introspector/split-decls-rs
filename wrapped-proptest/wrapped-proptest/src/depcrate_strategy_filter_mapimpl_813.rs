// Generated macro for impl_813 (impl)
macro_rules! Depcrate_strategy_filter_mapimpl_813 {
() => {
// Module: crate::strategy::filter_map
// Provides: {"impl_813"}
// Dependencies: {}
impl < V : Clone + ValueTree , F : Fn (V :: Value) -> Option < O > , O > Clone for FilterMapValueTree < V , F , O > { fn clone (& self) -> Self { Self :: new (self . source . clone () , & self . fun , self . fresh_current ()) } }
};
}
