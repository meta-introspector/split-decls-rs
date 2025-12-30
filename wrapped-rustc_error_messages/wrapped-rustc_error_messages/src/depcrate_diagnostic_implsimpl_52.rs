// Generated macro for impl_52 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_52 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_52"}
// Dependencies: {}
impl IntoDiagArg for Backtrace { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: from (self . to_string ())) } }
};
}
