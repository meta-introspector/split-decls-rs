// Generated macro for impl_47 (impl)
macro_rules! Depcrate_to_tokensimpl_47 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_47"}
// Dependencies: {}
impl ToTokens for u8 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: u8_suffixed (* self)) ; } }
};
}
