// Generated macro for impl_239 (impl)
macro_rules! Depcrate_diagnostics_region_nameimpl_239 {
() => {
// Module: crate::diagnostics::region_name
// Provides: {"impl_239"}
// Dependencies: {}
impl rustc_errors :: IntoDiagArg for RegionName { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> rustc_errors :: DiagArgValue { self . to_string () . into_diag_arg (path) } }
};
}
