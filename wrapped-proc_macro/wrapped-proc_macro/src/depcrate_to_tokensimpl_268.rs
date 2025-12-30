// Generated macro for impl_268 (impl)
macro_rules! Depcrate_to_tokensimpl_268 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_268"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for u8 { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: u8_suffixed (* self) . to_tokens (tokens) } }
};
}
