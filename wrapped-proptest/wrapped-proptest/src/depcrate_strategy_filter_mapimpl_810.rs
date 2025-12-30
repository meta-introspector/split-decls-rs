// Generated macro for impl_810 (impl)
macro_rules! Depcrate_strategy_filter_mapimpl_810 {
() => {
// Module: crate::strategy::filter_map
// Provides: {"impl_810"}
// Dependencies: {}
impl < S : Clone , F > Clone for FilterMap < S , F > { fn clone (& self) -> Self { Self { source : self . source . clone () , whence : self . whence . clone () , fun : Arc :: clone (& self . fun) , } } }
};
}
