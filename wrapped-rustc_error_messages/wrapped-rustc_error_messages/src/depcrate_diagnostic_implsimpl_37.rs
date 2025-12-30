// Generated macro for impl_37 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_37 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_37"}
// Dependencies: {}
impl IntoDiagArg for char { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (format ! ("{self:?}"))) } }
};
}
