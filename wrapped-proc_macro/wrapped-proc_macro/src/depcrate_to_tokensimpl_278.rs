// Generated macro for impl_278 (impl)
macro_rules! Depcrate_to_tokensimpl_278 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_278"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for f32 { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: f32_suffixed (* self) . to_tokens (tokens) } }
};
}
