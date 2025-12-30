// Generated macro for parse_variable_definition (function)
macro_rules! Depcrate_parser_documentparse_variable_definition {
() => {
// Module: crate::parser::document
// Provides: {"parse_variable_definition"}
// Dependencies: {}
fn parse_variable_definition < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > ,) -> ParseResult < (Spanning < & 'a str > , VariableDefinition < 'a , S >) > where S : ScalarValue , { let description = parse_description (parser) ? ; let start_pos = parser . expect (& Token :: Dollar) ? . span . start ; let var_name = parser . expect_name () ? ; parser . expect (& Token :: Colon) ? ; let var_type = parse_type (parser) ? ; let tpe = schema . lookup_type (& var_type . item) ; let default_value = if parser . skip (& Token :: Equals) ? . is_some () { Some (parse_value_literal (parser , true , schema , tpe) ?) } else { None } ; let directives = parse_directives (parser , schema) ? ; Ok (Spanning :: start_end (& start_pos , & default_value . as_ref () . map_or (& var_type . span . end , | s | & s . span . end) . clone () , (Spanning :: start_end (& start_pos , & var_name . span . end , var_name . item) , VariableDefinition { description , var_type , default_value , directives : directives . map (| s | s . item) , } ,) ,)) }
};
}
