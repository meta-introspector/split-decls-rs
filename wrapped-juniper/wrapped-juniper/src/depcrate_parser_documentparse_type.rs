// Generated macro for parse_type (function)
macro_rules! Depcrate_parser_documentparse_type {
() => {
// Module: crate::parser::document
// Provides: {"parse_type"}
// Dependencies: {}
pub fn parse_type < 'a > (parser : & mut Parser < 'a >) -> ParseResult < Type < & 'a str > > { let parsed_type = if let Some (Spanning { span : start_span , .. }) = parser . skip (& Token :: BracketOpen) ? { let inner_type = parse_type (parser) ? ; let end_pos = parser . expect (& Token :: BracketClose) ? . span . end ; Spanning :: start_end (& start_span . start , & end_pos , inner_type . item . wrap_list (None)) } else { parser . expect_name () ? . map (Type :: nullable) } ; Ok (match * parser . peek () { Spanning { item : Token :: ExclamationMark , .. } => wrap_non_null (parser , parsed_type) ? , _ => parsed_type , }) }
};
}
