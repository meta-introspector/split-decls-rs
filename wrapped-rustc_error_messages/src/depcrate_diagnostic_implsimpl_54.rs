// Generated macro for impl_54 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_54 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_54"}
// Dependencies: {}
impl IntoDiagArg for ast :: FloatTy { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . name_str ())) } }
};
}
