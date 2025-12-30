// Generated macro for parse_selection (function)
macro_rules! Depcrate_parser_documentparse_selection {
() => {
// Module: crate::parser::document
// Provides: {"parse_selection"}
// Dependencies: {}
fn parse_selection < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > , fields : Option < & [& MetaField < S >] > ,) -> UnlocatedParseResult < Selection < 'a , S > > where S : ScalarValue , { match parser . peek () . item { Token :: Ellipsis => parse_fragment (parser , schema , fields) , _ => parse_field (parser , schema , fields) . map (Selection :: Field) , } }
};
}
