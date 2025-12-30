// Generated macro for impl_43 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_43 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'a > IntoDiagArg for & 'a Path { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . display () . to_string ())) } }
};
}
