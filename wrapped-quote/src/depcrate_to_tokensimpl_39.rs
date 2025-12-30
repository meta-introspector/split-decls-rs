// Generated macro for impl_39 (impl)
macro_rules! Depcrate_to_tokensimpl_39 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_39"}
// Dependencies: {}
impl ToTokens for str { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: string (self)) ; } }
};
}
