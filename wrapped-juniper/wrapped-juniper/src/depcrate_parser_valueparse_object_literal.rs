// Generated macro for parse_object_literal (function)
macro_rules! Depcrate_parser_valueparse_object_literal {
() => {
// Module: crate::parser::value
// Provides: {"parse_object_literal"}
// Dependencies: {}
fn parse_object_literal < S > (parser : & mut Parser < '_ > , is_const : bool , schema : & SchemaType < S > , object_tpe : Option < & InputObjectMeta < S > > ,) -> ParseResult < InputValue < S > > where S : ScalarValue , { Ok (parser . delimited_list (& Token :: CurlyOpen , | p | parse_object_field (p , is_const , schema , object_tpe) , & Token :: CurlyClose ,) ? . map (| items | InputValue :: parsed_object (items . into_iter () . map (| s | s . item) . collect ()))) }
};
}
