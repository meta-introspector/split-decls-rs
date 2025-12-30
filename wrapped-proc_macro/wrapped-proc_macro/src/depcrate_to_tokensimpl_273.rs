// Generated macro for impl_273 (impl)
macro_rules! Depcrate_to_tokensimpl_273 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_273"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for i8 { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: i8_suffixed (* self) . to_tokens (tokens) } }
};
}
