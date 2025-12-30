// Generated macro for impl_886 (impl)
macro_rules! Depcrate_strategy_mapimpl_886 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_886"}
// Dependencies: {}
impl < S : Clone , F > Clone for Map < S , F > { fn clone (& self) -> Self { Map { source : self . source . clone () , fun : Arc :: clone (& self . fun) , } } }
};
}
