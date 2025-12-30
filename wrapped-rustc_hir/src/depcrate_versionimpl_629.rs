// Generated macro for impl_629 (impl)
macro_rules! Depcrate_versionimpl_629 {
() => {
// Module: crate::version
// Provides: {"impl_629"}
// Dependencies: {}
impl IntoDiagArg for RustcVersion { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string ())) } }
};
}
