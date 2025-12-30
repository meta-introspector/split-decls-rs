// Generated macro for impl_55 (impl)
macro_rules! Depcrate_to_tokensimpl_55 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_55"}
// Dependencies: {}
impl ToTokens for char { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: character (* self)) ; } }
};
}
