// Generated macro for impl_169 (impl)
macro_rules! Depcrate_tokens_to_tokensimpl_169 {
() => {
// Module: crate::tokens::to_tokens
// Provides: {"impl_169"}
// Dependencies: {}
impl ToTokens for Literal { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . push_str (self . as_str ()) ; } }
};
}
