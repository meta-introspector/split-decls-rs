// Generated macro for parse_directives (function)
macro_rules! Depcrate_parser_documentparse_directives {
() => {
// Module: crate::parser::document
// Provides: {"parse_directives"}
// Dependencies: {}
fn parse_directives < 'a , S > (parser : & mut Parser < 'a > , schema : & SchemaType < S > ,) -> OptionParseResult < Vec < Spanning < Directive < 'a , S > > > > where S : ScalarValue , { if parser . peek () . item != Token :: At { Ok (None) } else { let mut items = Vec :: new () ; while parser . peek () . item == Token :: At { items . push (parse_directive (parser , schema) ?) ; } Ok (Spanning :: spanning (items)) } }
};
}
