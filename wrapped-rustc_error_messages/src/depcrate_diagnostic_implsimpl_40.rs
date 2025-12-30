// Generated macro for impl_40 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_40 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a > IntoDiagArg for & 'a str { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_string () . into_diag_arg (path) } }
};
}
