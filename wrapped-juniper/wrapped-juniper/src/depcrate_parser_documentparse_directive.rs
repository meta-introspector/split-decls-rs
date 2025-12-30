// Generated macro for parse_directive (function)
macro_rules! Depcrate_parser_documentparse_directive {
() => {
// Module: crate::parser::document
// Provides: {"parse_directive"}
// Dependencies: {}
fn parse_directive < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > ,) -> ParseResult < Directive < 'a , S > > where S : ScalarValue , { let start_pos = parser . expect (& Token :: At) ? . span . start ; let name = parser . expect_name () ? ; let directive = schema . directive_by_name (name . item) ; let arguments = parse_arguments (parser , schema , directive . as_ref () . map (| d | & d . arguments as & [_]) ,) ? ; Ok (Spanning :: start_end (& start_pos , & arguments . as_ref () . map_or (& name . span . end , | s | & s . span . end) . clone () , Directive { name , arguments } ,)) }
};
}
