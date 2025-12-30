// Generated macro for impl_888 (impl)
macro_rules! Depcrate_strategy_mapimpl_888 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_888"}
// Dependencies: {}
impl < S : ValueTree , O : fmt :: Debug , F : Fn (S :: Value) -> O > ValueTree for Map < S , F > { type Value = O ; fn current (& self) -> O { (self . fun) (self . source . current ()) } fn simplify (& mut self) -> bool { self . source . simplify () } fn complicate (& mut self) -> bool { self . source . complicate () } }
};
}
