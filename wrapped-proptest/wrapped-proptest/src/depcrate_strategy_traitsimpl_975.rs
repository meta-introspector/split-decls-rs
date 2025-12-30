// Generated macro for impl_975 (impl)
macro_rules! Depcrate_strategy_traitsimpl_975 {
() => {
// Module: crate::strategy::traits
// Provides: {"impl_975"}
// Dependencies: {}
impl < T : ValueTree > ValueTree for NoShrink < T > { type Value = T :: Value ; fn current (& self) -> T :: Value { self . 0 . current () } fn simplify (& mut self) -> bool { false } fn complicate (& mut self) -> bool { false } }
};
}
