// Generated macro for impl_50 (impl)
macro_rules! Depcrate_to_tokensimpl_50 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_50"}
// Dependencies: {}
impl ToTokens for u64 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: u64_suffixed (* self)) ; } }
};
}
