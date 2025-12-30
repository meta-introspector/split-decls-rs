// Generated macro for impl_45 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_45 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_45"}
// Dependencies: {}
impl IntoDiagArg for ast :: Expr { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (pprust :: expr_to_string (& self))) } }
};
}
