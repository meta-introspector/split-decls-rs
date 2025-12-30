// Generated macro for impl_277 (impl)
macro_rules! Depcrate_to_tokensimpl_277 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_277"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for i128 { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: i128_suffixed (* self) . to_tokens (tokens) } }
};
}
