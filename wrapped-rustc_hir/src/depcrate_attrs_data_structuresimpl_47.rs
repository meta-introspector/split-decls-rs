// Generated macro for impl_47 (impl)
macro_rules! Depcrate_attrs_data_structuresimpl_47 {
() => {
// Module: crate::attrs::data_structures
// Provides: {"impl_47"}
// Dependencies: {}
impl IntoDiagArg for MirPhase { fn into_diag_arg (self , _path : & mut Option < PathBuf >) -> DiagArgValue { let arg = match self { MirPhase :: Initial => "initial" , MirPhase :: PostCleanup => "post-cleanup" , MirPhase :: Optimized => "optimized" , } ; DiagArgValue :: Str (Cow :: Borrowed (arg)) } }
};
}
