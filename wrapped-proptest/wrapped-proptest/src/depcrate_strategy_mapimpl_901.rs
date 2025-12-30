// Generated macro for impl_901 (impl)
macro_rules! Depcrate_strategy_mapimpl_901 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_901"}
// Dependencies: {}
impl < S : Clone , F > Clone for PerturbValueTree < S , F > { fn clone (& self) -> Self { PerturbValueTree { source : self . source . clone () , fun : Arc :: clone (& self . fun) , rng : self . rng . clone () , } } }
};
}
