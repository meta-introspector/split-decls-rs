// Generated macro for impl_48 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_48 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_48"}
// Dependencies: {}
impl IntoDiagArg for ast :: token :: TokenKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (pprust :: token_kind_to_string (& self)) } }
};
}
