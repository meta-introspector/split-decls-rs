// Generated macro for impl_1072 (impl)
macro_rules! Depcrate_strategy_staticsimpl_1072 {
() => {
// Module: crate::strategy::statics
// Provides: {"impl_1072"}
// Dependencies: {}
impl < S : ValueTree , F : FilterFn < S :: Value > > ValueTree for Filter < S , F > { type Value = S :: Value ; fn current (& self) -> S :: Value { self . source . current () } fn simplify (& mut self) -> bool { if self . source . simplify () { self . ensure_acceptable () ; true } else { false } } fn complicate (& mut self) -> bool { if self . source . complicate () { self . ensure_acceptable () ; true } else { false } } }
};
}
