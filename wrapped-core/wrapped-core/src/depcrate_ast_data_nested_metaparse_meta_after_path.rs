// Generated macro for parse_meta_after_path (function)
macro_rules! Depcrate_ast_data_nested_metaparse_meta_after_path {
() => {
// Module: crate::ast::data::nested_meta
// Provides: {"parse_meta_after_path"}
// Dependencies: {}
fn parse_meta_after_path < 'a > (path : Path , input : ParseStream < 'a >) -> syn :: Result < Meta > { if input . peek (token :: Paren) || input . peek (token :: Bracket) || input . peek (token :: Brace) { parse_meta_list_after_path (path , input) . map (Meta :: List) } else if input . peek (Token ! [=]) { parse_meta_name_value_after_path (path , input) . map (Meta :: NameValue) } else { Ok (Meta :: Path (path)) } }
};
}
