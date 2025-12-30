// Generated macro for parse_selection_set (function)
macro_rules! Depcrate_parser_documentparse_selection_set {
() => {
// Module: crate::parser::document
// Provides: {"parse_selection_set"}
// Dependencies: {}
fn parse_selection_set < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > , fields : Option < & [& MetaField < S >] > ,) -> ParseResult < Vec < Selection < 'a , S > > > where S : ScalarValue , { parser . unlocated_delimited_nonempty_list (& Token :: CurlyOpen , | p | parse_selection (p , schema , fields) , & Token :: CurlyClose ,) }
};
}
