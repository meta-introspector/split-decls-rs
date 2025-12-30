// Generated macro for impl_547 (impl)
macro_rules! Depcrate_limitimpl_547 {
() => {
// Module: crate::limit
// Provides: {"impl_547"}
// Dependencies: {}
impl IntoDiagArg for Limit { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_string () . into_diag_arg (& mut None) } }
};
}
