// Generated macro for impl_199 (impl)
macro_rules! Depcrate_contextimpl_199 {
() => {
// Module: crate::context
// Provides: {"impl_199"}
// Dependencies: {}
impl DynLintStore for LintStore { fn lint_groups_iter (& self) -> Box < dyn Iterator < Item = rustc_session :: LintGroup > + '_ > { Box :: new (self . get_lint_groups () . map (| (name , lints , is_externally_loaded) | { rustc_session :: LintGroup { name , lints , is_externally_loaded } })) } }
};
}
