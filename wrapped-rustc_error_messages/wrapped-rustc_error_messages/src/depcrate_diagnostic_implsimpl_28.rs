// Generated macro for impl_28 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_28 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_28"}
// Dependencies: {}
impl IntoDiagArg for DiagArgFromDisplay < '_ > { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . 0 . to_string () . into_diag_arg (path) } }
};
}
