// Generated macro for impl_41 (impl)
macro_rules! Depcrate_to_tokensimpl_41 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_41"}
// Dependencies: {}
impl ToTokens for i8 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: i8_suffixed (* self)) ; } }
};
}
