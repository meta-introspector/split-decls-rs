// Generated macro for impl_127 (impl)
macro_rules! Depcrate_astimpl_127 {
() => {
// Module: crate::ast
// Provides: {"impl_127"}
// Dependencies: {}
impl PrefixHandle { fn is_template_prefix (& self) -> bool { match * self { PrefixHandle :: BackReference (_) | PrefixHandle :: WellKnown (_) => true , PrefixHandle :: NonSubstitution (_) => false , } } }
};
}
