// Generated macro for parse_definition (function)
macro_rules! Depcrate_parser_documentparse_definition {
() => {
// Module: crate::parser::document
// Provides: {"parse_definition"}
// Dependencies: {}
fn parse_definition < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > ,) -> UnlocatedParseResult < Definition < 'a , S > > where S : ScalarValue , { let description = parse_description (parser) ? ; let mut def = match parser . peek () . item { Token :: CurlyOpen if description . is_some () => { return Err (parser . next_token () ? . map (ParseError :: unexpected_token)) ; } Token :: CurlyOpen | Token :: Name ("query") | Token :: Name ("mutation") | Token :: Name ("subscription") => { Definition :: Operation (parse_operation_definition (parser , schema) ?) } Token :: Name ("fragment") => Definition :: Fragment (parse_fragment_definition (parser , schema) ?) , _ => return Err (parser . next_token () ? . map (ParseError :: unexpected_token)) , } ; def . set_description (description) ; Ok (def) }
};
}
