// Generated macro for wrap_non_null (function)
macro_rules! Depcrate_parser_documentwrap_non_null {
() => {
// Module: crate::parser::document
// Provides: {"wrap_non_null"}
// Dependencies: {}
fn wrap_non_null < 'a > (parser : & mut Parser < 'a > , mut inner : Spanning < Type < & 'a str > > ,) -> ParseResult < Type < & 'a str > > { let end_pos = & parser . expect (& Token :: ExclamationMark) ? . span . end ; if ! inner . item . is_non_null () { inner . item = inner . item . wrap_non_null () ; } Ok (Spanning :: start_end (& inner . span . start , end_pos , inner . item)) }
};
}
