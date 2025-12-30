// Generated macro for impl_746 (impl)
macro_rules! Depcrate_errorsimpl_746 {
() => {
// Module: crate::errors
// Provides: {"impl_746"}
// Dependencies: {}
impl IntoDiagArg for ExpectedPointerMutability { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { match self { ExpectedPointerMutability :: Mut => DiagArgValue :: Str (Cow :: Borrowed ("*mut")) , ExpectedPointerMutability :: Not => DiagArgValue :: Str (Cow :: Borrowed ("*_")) , } } }
};
}
