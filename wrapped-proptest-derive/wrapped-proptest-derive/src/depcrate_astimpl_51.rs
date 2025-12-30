// Generated macro for impl_51 (impl)
macro_rules! Depcrate_astimpl_51 {
() => {
// Module: crate::ast
// Provides: {"impl_51"}
// Dependencies: {}
impl ToTokens for FromReg { fn to_tokens (& self , tokens : & mut TokenStream) { match self { FromReg :: Top => call_site_ident (TOP_PARAM_NAME) . to_tokens (tokens) , FromReg :: Num (reg) => param (* reg) . to_tokens (tokens) , } } }
};
}
