// Generated macro for impl_47 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_47 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_47"}
// Dependencies: {}
impl IntoDiagArg for ast :: token :: Token { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (pprust :: token_to_string (& self)) } }
};
}
