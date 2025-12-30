// Generated macro for parse_document (function)
macro_rules! Depcrate_parser_documentparse_document {
() => {
// Module: crate::parser::document
// Provides: {"parse_document"}
// Dependencies: {}
fn parse_document < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > ,) -> UnlocatedParseResult < OwnedDocument < 'a , S > > where S : ScalarValue , { let mut defs = Vec :: new () ; loop { defs . push (parse_definition (parser , schema) ?) ; if parser . peek () . item == Token :: EndOfFile { return Ok (defs) ; } } }
};
}
