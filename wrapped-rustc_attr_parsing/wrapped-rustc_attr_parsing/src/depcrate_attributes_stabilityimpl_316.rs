// Generated macro for impl_316 (impl)
macro_rules! Depcrate_attributes_stabilityimpl_316 {
() => {
// Module: crate::attributes::stability
// Provides: {"impl_316"}
// Dependencies: {}
impl StabilityParser { # [doc = " Checks, and emits an error when a stability (or unstability) was already set, which would be a duplicate."] fn check_duplicate < S : Stage > (& self , cx : & AcceptContext < '_ , '_ , S >) -> bool { if let Some ((_ , _)) = self . stability { cx . emit_err (session_diagnostics :: MultipleStabilityLevels { span : cx . attr_span }) ; true } else { false } } }
};
}
