// Generated macro for parse_optional_selection_set (function)
macro_rules! Depcrate_parser_documentparse_optional_selection_set {
() => {
// Module: crate::parser::document
// Provides: {"parse_optional_selection_set"}
// Dependencies: {}
fn parse_optional_selection_set < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > , fields : Option < & [& MetaField < S >] > ,) -> OptionParseResult < Vec < Selection < 'a , S > > > where S : ScalarValue , { if parser . peek () . item == Token :: CurlyOpen { Ok (Some (parse_selection_set (parser , schema , fields) ?)) } else { Ok (None) } }
};
}
