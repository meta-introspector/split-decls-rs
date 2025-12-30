// Generated macro for impl_798 (impl)
macro_rules! Depcrate_strategy_filterimpl_798 {
() => {
// Module: crate::strategy::filter
// Provides: {"impl_798"}
// Dependencies: {}
impl < S : Clone , F > Clone for Filter < S , F > { fn clone (& self) -> Self { Filter { source : self . source . clone () , whence : "unused" . into () , fun : Arc :: clone (& self . fun) , } } }
};
}
