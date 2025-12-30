// Generated macro for impl_816 (impl)
macro_rules! Depcrate_strategy_filter_mapimpl_816 {
() => {
// Module: crate::strategy::filter_map
// Provides: {"impl_816"}
// Dependencies: {}
impl < V : ValueTree , F : Fn (V :: Value) -> Option < O > , O : fmt :: Debug > ValueTree for FilterMapValueTree < V , F , O > { type Value = O ; fn current (& self) -> O { if let Some (current) = self . current . replace (None) { current } else { self . fresh_current () } } fn simplify (& mut self) -> bool { if self . source . simplify () { self . ensure_acceptable () ; true } else { false } } fn complicate (& mut self) -> bool { if self . source . complicate () { self . ensure_acceptable () ; true } else { false } } }
};
}
