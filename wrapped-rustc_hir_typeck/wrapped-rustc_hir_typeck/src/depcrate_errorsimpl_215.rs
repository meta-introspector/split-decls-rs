// Generated macro for impl_215 (impl)
macro_rules! Depcrate_errorsimpl_215 {
() => {
// Module: crate::errors
// Provides: {"impl_215"}
// Dependencies: {}
impl IntoDiagArg for ReturnLikeStatementKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { let kind = match self { Self :: Return => "return" , Self :: Become => "become" , } . into () ; DiagArgValue :: Str (kind) } }
};
}
