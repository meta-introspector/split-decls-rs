// Generated macro for impl_168 (impl)
macro_rules! Depcrate_tokens_to_tokensimpl_168 {
() => {
// Module: crate::tokens::to_tokens
// Provides: {"impl_168"}
// Dependencies: {}
impl ToTokens for bool { fn to_tokens (& self , tokens : & mut TokenStream) { let word = if * self { "true" } else { "false" } ; tokens . push_space () ; tokens . push_str (word) ; } }
};
}
