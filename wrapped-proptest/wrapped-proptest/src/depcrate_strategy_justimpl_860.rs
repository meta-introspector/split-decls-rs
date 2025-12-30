// Generated macro for impl_860 (impl)
macro_rules! Depcrate_strategy_justimpl_860 {
() => {
// Module: crate::strategy::just
// Provides: {"impl_860"}
// Dependencies: {}
impl < T : fmt :: Debug , F : Fn () -> T > ValueTree for LazyJust < T , F > { type Value = T ; noshrink ! () ; fn current (& self) -> Self :: Value { (self . function) () } }
};
}
