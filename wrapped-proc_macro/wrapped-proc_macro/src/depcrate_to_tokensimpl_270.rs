// Generated macro for impl_270 (impl)
macro_rules! Depcrate_to_tokensimpl_270 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_270"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for u32 { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: u32_suffixed (* self) . to_tokens (tokens) } }
};
}
