// Generated macro for impl_897 (impl)
macro_rules! Depcrate_strategy_mapimpl_897 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_897"}
// Dependencies: {}
impl < S : Clone , F > Clone for Perturb < S , F > { fn clone (& self) -> Self { Perturb { source : self . source . clone () , fun : Arc :: clone (& self . fun) , } } }
};
}
