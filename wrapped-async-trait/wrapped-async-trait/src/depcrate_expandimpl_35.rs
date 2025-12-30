// Generated macro for impl_35 (impl)
macro_rules! Depcrate_expandimpl_35 {
() => {
// Module: crate::expand
// Provides: {"impl_35"}
// Dependencies: {}
impl ToTokens for Item { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Item :: Trait (item) => item . to_tokens (tokens) , Item :: Impl (item) => item . to_tokens (tokens) , } } }
};
}
