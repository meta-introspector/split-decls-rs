// Generated macro for impl_25 (impl)
macro_rules! Depcrate_ast_data_nested_metaimpl_25 {
() => {
// Module: crate::ast::data::nested_meta
// Provides: {"impl_25"}
// Dependencies: {}
impl NestedMeta { pub fn parse_meta_list (tokens : TokenStream) -> syn :: Result < Vec < Self > > { syn :: punctuated :: Punctuated :: < NestedMeta , Token ! [,] > :: parse_terminated . parse2 (tokens) . map (| punctuated | punctuated . into_iter () . collect ()) } }
};
}
