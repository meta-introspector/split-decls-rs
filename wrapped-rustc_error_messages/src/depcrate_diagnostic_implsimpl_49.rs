// Generated macro for impl_49 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_49 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_49"}
// Dependencies: {}
impl IntoDiagArg for std :: ffi :: CString { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string_lossy () . into_owned ())) } }
};
}
