// Generated macro for parse_field (function)
macro_rules! Depcrate_parser_documentparse_field {
() => {
// Module: crate::parser::document
// Provides: {"parse_field"}
// Dependencies: {}
fn parse_field < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > , fields : Option < & [& MetaField < S >] > ,) -> ParseResult < Field < 'a , S > > where S : ScalarValue , { let mut alias = Some (parser . expect_name () ?) ; let name = if parser . skip (& Token :: Colon) ? . is_some () { parser . expect_name () ? } else { alias . take () . unwrap () } ; let field = fields . and_then (| f | f . iter () . find (| f | f . name == name . item)) ; let args = field . as_ref () . and_then (| f | f . arguments . as_ref () . map (| a | a as & [_])) ; let fields = field . as_ref () . and_then (| f | schema . lookup_type (& f . field_type)) . and_then (| m | m . fields (schema)) ; let fields = fields . as_ref () . map (| c | c as & [_]) ; let arguments = parse_arguments (parser , schema , args) ? ; let directives = parse_directives (parser , schema) ? ; let selection_set = parse_optional_selection_set (parser , schema , fields) ? ; Ok (Spanning :: start_end (& alias . as_ref () . unwrap_or (& name) . span . start , & selection_set . as_ref () . map (| s | & s . span . end) . or_else (| | directives . as_ref () . map (| s | & s . span . end)) . or_else (| | arguments . as_ref () . map (| s | & s . span . end)) . unwrap_or (& name . span . end) . clone () , Field { alias , name , arguments , directives : directives . map (| s | s . item) , selection_set : selection_set . map (| s | s . item) , } ,)) }
};
}
