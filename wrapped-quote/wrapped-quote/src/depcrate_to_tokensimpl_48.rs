// Generated macro for impl_48 (impl)
macro_rules! Depcrate_to_tokensimpl_48 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_48"}
// Dependencies: {}
impl ToTokens for u16 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: u16_suffixed (* self)) ; } }
};
}
