// Generated macro for impl_39 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_39 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_39"}
// Dependencies: {}
impl IntoDiagArg for rustc_span :: Symbol { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_ident_string () . into_diag_arg (path) } }
};
}
