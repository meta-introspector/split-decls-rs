// Generated macro for impl_276 (impl)
macro_rules! Depcrate_to_tokensimpl_276 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_276"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for i64 { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: i64_suffixed (* self) . to_tokens (tokens) } }
};
}
