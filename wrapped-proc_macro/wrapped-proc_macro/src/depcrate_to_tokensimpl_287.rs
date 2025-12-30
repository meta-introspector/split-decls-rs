// Generated macro for impl_287 (impl)
macro_rules! Depcrate_to_tokensimpl_287 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_287"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for CString { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: c_string (self) . to_tokens (tokens) } }
};
}
