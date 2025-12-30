// Generated macro for impl_41 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_41 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_41"}
// Dependencies: {}
impl IntoDiagArg for String { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self)) } }
};
}
