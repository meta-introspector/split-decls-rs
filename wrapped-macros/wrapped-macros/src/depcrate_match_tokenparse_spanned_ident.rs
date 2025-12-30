// Generated macro for parse_spanned_ident (function)
macro_rules! Depcrate_match_tokenparse_spanned_ident {
() => {
// Module: crate::match_token
// Provides: {"parse_spanned_ident"}
// Dependencies: {}
fn parse_spanned_ident (parser : & mut Parser) -> Result < ast :: SpannedIdent , FatalError > { let lo = parser . span . lo ; let ident = try ! (parser . parse_ident ()) ; let hi = parser . last_span . hi ; Ok (spanned (lo , hi , ident)) }
};
}
