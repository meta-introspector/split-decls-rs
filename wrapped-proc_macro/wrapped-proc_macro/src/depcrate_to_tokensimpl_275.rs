// Generated macro for impl_275 (impl)
macro_rules! Depcrate_to_tokensimpl_275 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_275"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for i32 { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: i32_suffixed (* self) . to_tokens (tokens) } }
};
}
