// Generated macro for impl_272 (impl)
macro_rules! Depcrate_to_tokensimpl_272 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_272"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for u128 { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: u128_suffixed (* self) . to_tokens (tokens) } }
};
}
