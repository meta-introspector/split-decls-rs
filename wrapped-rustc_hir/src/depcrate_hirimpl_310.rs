// Generated macro for impl_310 (impl)
macro_rules! Depcrate_hirimpl_310 {
() => {
// Module: crate::hir
// Provides: {"impl_310"}
// Dependencies: {}
impl IntoDiagArg for ConstContext { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (match self { ConstContext :: ConstFn => "const_fn" , ConstContext :: Static (_) => "static" , ConstContext :: Const { .. } => "const" , })) } }
};
}
