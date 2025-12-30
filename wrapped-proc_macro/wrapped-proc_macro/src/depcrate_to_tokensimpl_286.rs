// Generated macro for impl_286 (impl)
macro_rules! Depcrate_to_tokensimpl_286 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_286"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for CStr { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: c_string (self) . to_tokens (tokens) } }
};
}
