// Generated macro for impl_379 (impl)
macro_rules! Depcrate_errorsimpl_379 {
() => {
// Module: crate::errors
// Provides: {"impl_379"}
// Dependencies: {}
impl rustc_errors :: IntoDiagArg for InternKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (match self { InternKind :: Static (Mutability :: Not) => "static" , InternKind :: Static (Mutability :: Mut) => "static_mut" , InternKind :: Constant => "const" , InternKind :: Promoted => "promoted" , })) } }
};
}
