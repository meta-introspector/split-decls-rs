// Generated macro for impl_982 (impl)
macro_rules! Depcrate_strategy_traitsimpl_982 {
() => {
// Module: crate::strategy::traits
// Provides: {"impl_982"}
// Dependencies: {}
impl < T : ValueTree + ? Sized > ValueTree for Box < T > { type Value = T :: Value ; fn current (& self) -> Self :: Value { (* * self) . current () } fn simplify (& mut self) -> bool { (* * self) . simplify () } fn complicate (& mut self) -> bool { (* * self) . complicate () } }
};
}
