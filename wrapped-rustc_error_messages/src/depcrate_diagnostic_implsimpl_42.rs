// Generated macro for impl_42 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_42 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a > IntoDiagArg for Cow < 'a , str > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . into_owned ())) } }
};
}
