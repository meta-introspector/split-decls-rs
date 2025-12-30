// Generated macro for parse_argument (function)
macro_rules! Depcrate_parser_documentparse_argument {
() => {
// Module: crate::parser::document
// Provides: {"parse_argument"}
// Dependencies: {}
fn parse_argument < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > , arguments : Option < & [Argument < S >] > ,) -> ParseResult < (Spanning < & 'a str > , Spanning < InputValue < S > >) > where S : ScalarValue , { let name = parser . expect_name () ? ; let tpe = arguments . and_then (| args | args . iter () . find (| a | a . name == name . item)) . and_then (| arg | schema . lookup_type (& arg . arg_type)) ; parser . expect (& Token :: Colon) ? ; let value = parse_value_literal (parser , false , schema , tpe) ? ; Ok (Spanning :: start_end (& name . span . start , & value . span . end . clone () , (name , value) ,)) }
};
}
