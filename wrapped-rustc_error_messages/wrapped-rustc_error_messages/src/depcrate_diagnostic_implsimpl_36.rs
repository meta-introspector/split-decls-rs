// Generated macro for impl_36 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_36 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_36"}
// Dependencies: {}
impl IntoDiagArg for bool { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { if self { DiagArgValue :: Str (Cow :: Borrowed ("true")) } else { DiagArgValue :: Str (Cow :: Borrowed ("false")) } } }
};
}
