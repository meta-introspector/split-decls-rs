// Generated macro for parse_fragment_definition (function)
macro_rules! Depcrate_parser_documentparse_fragment_definition {
() => {
// Module: crate::parser::document
// Provides: {"parse_fragment_definition"}
// Dependencies: {}
fn parse_fragment_definition < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > ,) -> ParseResult < Fragment < 'a , S > > where S : ScalarValue , { let start_pos = parser . expect (& Token :: Name ("fragment")) ? . span . start ; let name = match parser . expect_name () { Ok (n) => { if n . item == "on" { return Err (n . map (| _ | ParseError :: UnexpectedToken ("on" . into ()))) ; } else { n } } Err (e) => return Err (e) , } ; parser . expect (& Token :: Name ("on")) ? ; let type_cond = parser . expect_name () ? ; let fields = schema . concrete_type_by_name (type_cond . item) . and_then (| m | m . fields (schema)) ; let fields = fields . as_ref () . map (| c | c as & [_]) ; let directives = parse_directives (parser , schema) ? ; let selection_set = parse_selection_set (parser , schema , fields) ? ; Ok (Spanning :: start_end (& start_pos , & selection_set . span . end , Fragment { name , description : None , type_condition : type_cond , directives : directives . map (| s | s . item) , selection_set : selection_set . item , } ,)) }
};
}
