// Generated macro for impl_44 (impl)
macro_rules! Depcrate_to_tokensimpl_44 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_44"}
// Dependencies: {}
impl ToTokens for i64 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: i64_suffixed (* self)) ; } }
};
}
