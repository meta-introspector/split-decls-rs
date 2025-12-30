// Generated macro for impl_801 (impl)
macro_rules! Depcrate_strategy_filterimpl_801 {
() => {
// Module: crate::strategy::filter
// Provides: {"impl_801"}
// Dependencies: {}
impl < S : ValueTree , F : Fn (& S :: Value) -> bool > ValueTree for Filter < S , F > { type Value = S :: Value ; fn current (& self) -> S :: Value { self . source . current () } fn simplify (& mut self) -> bool { if self . source . simplify () { self . ensure_acceptable () ; true } else { false } } fn complicate (& mut self) -> bool { if self . source . complicate () { self . ensure_acceptable () ; true } else { false } } }
};
}
