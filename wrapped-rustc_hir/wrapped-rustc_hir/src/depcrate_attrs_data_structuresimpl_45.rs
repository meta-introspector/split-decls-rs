// Generated macro for impl_45 (impl)
macro_rules! Depcrate_attrs_data_structuresimpl_45 {
() => {
// Module: crate::attrs::data_structures
// Provides: {"impl_45"}
// Dependencies: {}
impl IntoDiagArg for MirDialect { fn into_diag_arg (self , _path : & mut Option < PathBuf >) -> DiagArgValue { let arg = match self { MirDialect :: Analysis => "analysis" , MirDialect :: Built => "built" , MirDialect :: Runtime => "runtime" , } ; DiagArgValue :: Str (Cow :: Borrowed (arg)) } }
};
}
