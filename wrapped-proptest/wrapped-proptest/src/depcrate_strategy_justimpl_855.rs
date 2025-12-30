// Generated macro for impl_855 (impl)
macro_rules! Depcrate_strategy_justimpl_855 {
() => {
// Module: crate::strategy::just
// Provides: {"impl_855"}
// Dependencies: {}
impl < T : Clone + fmt :: Debug > ValueTree for Just < T > { type Value = T ; noshrink ! () ; fn current (& self) -> T { self . 0 . clone () } }
};
}
