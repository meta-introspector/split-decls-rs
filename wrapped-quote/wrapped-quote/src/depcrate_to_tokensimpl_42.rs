// Generated macro for impl_42 (impl)
macro_rules! Depcrate_to_tokensimpl_42 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_42"}
// Dependencies: {}
impl ToTokens for i16 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: i16_suffixed (* self)) ; } }
};
}
