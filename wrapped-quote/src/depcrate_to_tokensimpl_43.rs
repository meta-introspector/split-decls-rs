// Generated macro for impl_43 (impl)
macro_rules! Depcrate_to_tokensimpl_43 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_43"}
// Dependencies: {}
impl ToTokens for i32 { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: i32_suffixed (* self)) ; } }
};
}
