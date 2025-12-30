// Generated macro for impl_894 (impl)
macro_rules! Depcrate_strategy_mapimpl_894 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_894"}
// Dependencies: {}
impl < S : ValueTree , O : fmt :: Debug > ValueTree for MapInto < S , O > where S :: Value : Into < O > , { type Value = O ; fn current (& self) -> O { self . source . current () . into () } fn simplify (& mut self) -> bool { self . source . simplify () } fn complicate (& mut self) -> bool { self . source . complicate () } }
};
}
