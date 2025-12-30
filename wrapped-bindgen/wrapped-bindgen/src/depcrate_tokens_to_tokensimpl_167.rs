// Generated macro for impl_167 (impl)
macro_rules! Depcrate_tokens_to_tokensimpl_167 {
() => {
// Module: crate::tokens::to_tokens
// Provides: {"impl_167"}
// Dependencies: {}
impl ToTokens for char { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . push_space () ; tokens . push ('\'') ; tokens . push (* self) ; tokens . push ('\'') ; } }
};
}
