// Generated macro for impl_37 (impl)
macro_rules! Depcrate_to_tokensimpl_37 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_37"}
// Dependencies: {}
impl ToTokens for str { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append (Literal :: string (self)) ; } }
};
}
