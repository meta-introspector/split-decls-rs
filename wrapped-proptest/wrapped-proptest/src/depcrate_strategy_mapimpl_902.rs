// Generated macro for impl_902 (impl)
macro_rules! Depcrate_strategy_mapimpl_902 {
() => {
// Module: crate::strategy::map
// Provides: {"impl_902"}
// Dependencies: {}
impl < S : ValueTree , O : fmt :: Debug , F : Fn (S :: Value , TestRng) -> O > ValueTree for PerturbValueTree < S , F > { type Value = O ; fn current (& self) -> O { (self . fun) (self . source . current () , self . rng . clone ()) } fn simplify (& mut self) -> bool { self . source . simplify () } fn complicate (& mut self) -> bool { self . source . complicate () } }
};
}
