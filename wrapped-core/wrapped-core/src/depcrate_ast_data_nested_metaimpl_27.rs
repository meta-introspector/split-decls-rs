// Generated macro for impl_27 (impl)
macro_rules! Depcrate_ast_data_nested_metaimpl_27 {
() => {
// Module: crate::ast::data::nested_meta
// Provides: {"impl_27"}
// Dependencies: {}
impl ToTokens for NestedMeta { fn to_tokens (& self , tokens : & mut TokenStream) { match self { NestedMeta :: Meta (meta) => meta . to_tokens (tokens) , NestedMeta :: Lit (lit) => lit . to_tokens (tokens) , } } }
};
}
