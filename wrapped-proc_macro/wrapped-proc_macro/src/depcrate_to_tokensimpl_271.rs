// Generated macro for impl_271 (impl)
macro_rules! Depcrate_to_tokensimpl_271 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_271"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for u64 { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: u64_suffixed (* self) . to_tokens (tokens) } }
};
}
