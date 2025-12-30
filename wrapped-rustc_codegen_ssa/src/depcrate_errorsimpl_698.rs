// Generated macro for impl_698 (impl)
macro_rules! Depcrate_errorsimpl_698 {
() => {
// Module: crate::errors
// Provides: {"impl_698"}
// Dependencies: {}
impl IntoDiagArg for DebugArgPath < '_ > { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> rustc_errors :: DiagArgValue { DiagArgValue :: Str (Cow :: Owned (format ! ("{:?}" , self . 0))) } }
};
}
