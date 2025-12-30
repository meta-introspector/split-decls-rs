// Generated macro for impl_49 (impl)
macro_rules! Depcrate_to_tokensimpl_49 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_49"}
// Dependencies: {}
impl ToTokens for u32 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: u32_suffixed (* self)) ; } }
};
}
