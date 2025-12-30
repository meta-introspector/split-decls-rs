// Generated macro for impl_862 (impl)
macro_rules! Depcrate_strategy_justimpl_862 {
() => {
// Module: crate::strategy::just
// Provides: {"impl_862"}
// Dependencies: {}
impl < T , F : Clone + Fn () -> T > Clone for LazyJust < T , F > { fn clone (& self) -> Self { Self { function : self . function . clone () , } } }
};
}
