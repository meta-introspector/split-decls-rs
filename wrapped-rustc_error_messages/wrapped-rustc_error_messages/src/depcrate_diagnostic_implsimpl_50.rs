// Generated macro for impl_50 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_50 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_50"}
// Dependencies: {}
impl IntoDiagArg for rustc_data_structures :: small_c_str :: SmallCStr { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . to_string_lossy () . into_owned ())) } }
};
}
