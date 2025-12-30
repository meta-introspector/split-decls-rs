// Generated macro for parse_meta_list_after_path (function)
macro_rules! Depcrate_ast_data_nested_metaparse_meta_list_after_path {
() => {
// Module: crate::ast::data::nested_meta
// Provides: {"parse_meta_list_after_path"}
// Dependencies: {}
fn parse_meta_list_after_path < 'a > (path : Path , input : ParseStream < 'a >) -> syn :: Result < MetaList > { let (delimiter , tokens) = input . step (| cursor | { if let Some ((TokenTree :: Group (g) , rest)) = cursor . token_tree () { let span = g . delim_span () ; let delimiter = match g . delimiter () { Delimiter :: Parenthesis => MacroDelimiter :: Paren (Paren (span)) , Delimiter :: Brace => MacroDelimiter :: Brace (Brace (span)) , Delimiter :: Bracket => MacroDelimiter :: Bracket (Bracket (span)) , Delimiter :: None => { return Err (cursor . error ("expected delimiter")) ; } } ; Ok (((delimiter , g . stream ()) , rest)) } else { Err (cursor . error ("expected delimiter")) } }) ? ; Ok (MetaList { path , delimiter , tokens , }) }
};
}
