// Generated macro for impl_51 (impl)
macro_rules! Depcrate_to_tokensimpl_51 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_51"}
// Dependencies: {}
impl ToTokens for u128 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: u128_suffixed (* self)) ; } }
};
}
