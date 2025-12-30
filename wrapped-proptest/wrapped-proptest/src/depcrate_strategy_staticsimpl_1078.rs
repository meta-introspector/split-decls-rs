// Generated macro for impl_1078 (impl)
macro_rules! Depcrate_strategy_staticsimpl_1078 {
() => {
// Module: crate::strategy::statics
// Provides: {"impl_1078"}
// Dependencies: {}
impl < S : ValueTree , F : MapFn < S :: Value > > ValueTree for Map < S , F > { type Value = F :: Output ; fn current (& self) -> F :: Output { self . fun . apply (self . source . current ()) } fn simplify (& mut self) -> bool { self . source . simplify () } fn complicate (& mut self) -> bool { self . source . complicate () } }
};
}
