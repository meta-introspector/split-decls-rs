// Generated macro for impl_74 (impl)
macro_rules! Depcrate_mock_itemimpl_74 {
() => {
// Module: crate::mock_item
// Provides: {"impl_74"}
// Dependencies: {}
impl ToTokens for MockItem { fn to_tokens (& self , tokens : & mut TokenStream) { match self { MockItem :: Module (mod_) => mod_ . to_tokens (tokens) , MockItem :: Struct (s) => s . to_tokens (tokens) } } }
};
}
