// Generated macro for impl_51 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_51 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_51"}
// Dependencies: {}
impl IntoDiagArg for ast :: Visibility { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { let s = pprust :: vis_to_string (& self) ; let s = s . trim_end () . to_string () ; DiagArgValue :: Str (Cow :: Owned (s)) } }
};
}
