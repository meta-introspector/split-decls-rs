// Generated macro for impl_31 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_31 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a , T : Clone + IntoDiagArg > IntoDiagArg for & 'a T { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . clone () . into_diag_arg (path) } }
};
}
