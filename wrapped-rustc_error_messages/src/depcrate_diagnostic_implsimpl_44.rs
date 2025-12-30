// Generated macro for impl_44 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_44 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_44"}
// Dependencies: {}
impl IntoDiagArg for PathBuf { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . display () . to_string ())) } }
};
}
