// Generated macro for impl_163 (impl)
macro_rules! Depcrate_tokens_to_tokensimpl_163 {
() => {
// Module: crate::tokens::to_tokens
// Provides: {"impl_163"}
// Dependencies: {}
impl ToTokens for str { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . push_str (" \"") ; tokens . push_str (self) ; tokens . push ('"') ; } }
};
}
