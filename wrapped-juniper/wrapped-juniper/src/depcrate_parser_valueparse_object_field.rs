// Generated macro for parse_object_field (function)
macro_rules! Depcrate_parser_valueparse_object_field {
() => {
// Module: crate::parser::value
// Provides: {"parse_object_field"}
// Dependencies: {}
fn parse_object_field < S > (parser : & mut Parser < '_ > , is_const : bool , schema : & SchemaType < S > , object_meta : Option < & InputObjectMeta < S > > ,) -> ParseResult < (Spanning < String > , Spanning < InputValue < S > >) > where S : ScalarValue , { let key = parser . expect_name () ? ; let tpe = object_meta . and_then (| o | o . input_fields . iter () . find (| f | f . name == key . item)) . and_then (| f | schema . lookup_type (& f . arg_type)) ; parser . expect (& Token :: Colon) ? ; let value = parse_value_literal (parser , is_const , schema , tpe) ? ; Ok (Spanning :: start_end (& key . span . start , & value . span . end . clone () , (key . map (| s | s . to_owned ()) , value) ,)) }
};
}
