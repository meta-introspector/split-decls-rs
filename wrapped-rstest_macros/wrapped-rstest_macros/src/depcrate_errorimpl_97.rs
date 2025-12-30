// Generated macro for impl_97 (impl)
macro_rules! Depcrate_errorimpl_97 {
() => {
// Module: crate::error
// Provides: {"impl_97"}
// Dependencies: {}
impl quote :: ToTokens for ErrorsVec { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . extend (self . 0 . iter () . map (| e | e . to_compile_error ())) } }
};
}
