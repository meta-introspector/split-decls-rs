// Generated macro for impl_46 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_46 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_46"}
// Dependencies: {}
impl IntoDiagArg for ast :: Path { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (pprust :: path_to_string (& self))) } }
};
}
