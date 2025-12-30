// Generated macro for parse_meta_path (function)
macro_rules! Depcrate_ast_data_nested_metaparse_meta_path {
() => {
// Module: crate::ast::data::nested_meta
// Provides: {"parse_meta_path"}
// Dependencies: {}
fn parse_meta_path < 'a > (input : ParseStream < 'a >) -> syn :: Result < Path > { Ok (Path { leading_colon : input . parse () ? , segments : { let mut segments = Punctuated :: new () ; loop { if ! input . peek (Ident :: peek_any) { break ; } let ident = Ident :: parse_any (input) ? ; segments . push_value (PathSegment :: from (ident)) ; if ! input . peek (Token ! [::]) { break ; } let punct = input . parse () ? ; segments . push_punct (punct) ; } if segments . is_empty () { return Err (input . parse :: < Ident > () . unwrap_err ()) ; } else if segments . trailing_punct () { return Err (input . error ("expected path segment after `::`")) ; } segments } , }) }
};
}
