// Generated macro for impl_279 (impl)
macro_rules! Depcrate_to_tokensimpl_279 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_279"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for f64 { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: f64_suffixed (* self) . to_tokens (tokens) } }
};
}
