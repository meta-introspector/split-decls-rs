// Generated macro for impl_874 (impl)
macro_rules! Depcrate_strategy_lazyimpl_874 {
() => {
// Module: crate::strategy::lazy
// Provides: {"impl_874"}
// Dependencies: {}
impl < S : Strategy > Clone for LazyValueTree < S > where S :: Tree : Clone , { fn clone (& self) -> Self { Self { state : self . state . clone () , } } }
};
}
