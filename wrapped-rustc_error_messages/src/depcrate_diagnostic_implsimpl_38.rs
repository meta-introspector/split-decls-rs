// Generated macro for impl_38 (impl)
macro_rules! Depcrate_diagnostic_implsimpl_38 {
() => {
// Module: crate::diagnostic_impls
// Provides: {"impl_38"}
// Dependencies: {}
impl IntoDiagArg for Vec < char > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: StrListSepByAnd (self . into_iter () . map (| c | Cow :: Owned (format ! ("{c:?}"))) . collect () ,) } }
};
}
