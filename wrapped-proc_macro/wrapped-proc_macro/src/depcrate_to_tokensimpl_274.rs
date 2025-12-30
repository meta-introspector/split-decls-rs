// Generated macro for impl_274 (impl)
macro_rules! Depcrate_to_tokensimpl_274 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_274"}
// Dependencies: {}
# [unstable (feature = "proc_macro_totokens" , issue = "130977")] impl ToTokens for i16 { fn to_tokens (& self , tokens : & mut TokenStream) { Literal :: i16_suffixed (* self) . to_tokens (tokens) } }
};
}
