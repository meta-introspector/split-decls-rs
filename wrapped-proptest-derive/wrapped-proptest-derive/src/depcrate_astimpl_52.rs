// Generated macro for impl_52 (impl)
macro_rules! Depcrate_astimpl_52 {
() => {
// Module: crate::ast
// Provides: {"impl_52"}
// Dependencies: {}
impl ToTokens for ToReg { fn to_tokens (& self , tokens : & mut TokenStream) { match * self { ToReg :: Range (to) if to == 1 => param (0) . to_tokens (tokens) , ToReg :: Range (to) => { let params : Vec < _ > = (0 .. to) . map (param) . collect () ; NestedTuple (& params) . to_tokens (tokens) } ToReg :: API => call_site_ident (API_PARAM_NAME) . to_tokens (tokens) , } } }
};
}
