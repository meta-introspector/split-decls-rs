// Generated macro for impl_836 (impl)
macro_rules! Depcrate_strategy_flattenimpl_836 {
() => {
// Module: crate::strategy::flatten
// Provides: {"impl_836"}
// Dependencies: {}
impl < S : Clone , F > Clone for IndFlattenMap < S , F > { fn clone (& self) -> Self { IndFlattenMap { source : self . source . clone () , fun : Arc :: clone (& self . fun) , } } }
};
}
