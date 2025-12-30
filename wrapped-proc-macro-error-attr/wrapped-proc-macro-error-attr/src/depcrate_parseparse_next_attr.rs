// Generated macro for parse_next_attr (function)
macro_rules! Depcrate_parseparse_next_attr {
() => {
// Module: crate::parse
// Provides: {"parse_next_attr"}
// Dependencies: {}
fn parse_next_attr (input : & mut Peekable < impl Iterator < Item = TokenTree > > ,) -> Result < Option < Attribute > > { let shebang = match input . peek () { Some (TokenTree :: Punct (ref punct)) if punct . as_char () == '#' => input . next () . unwrap () , _ => return Ok (None) , } ; let group = match input . peek () { Some (TokenTree :: Group (ref group)) if group . delimiter () == Delimiter :: Bracket => { let res = group . clone () ; input . next () ; res } other => { let span = other . map_or (Span :: call_site () , TokenTree :: span) ; return Err (Error :: new (span , "expected `[`" . to_string ())) ; } } ; let path = match group . stream () . into_iter () . next () { Some (TokenTree :: Ident (ident)) => Some (ident) , _ => None , } ; Ok (Some (Attribute { shebang , group : TokenTree :: Group (group) , path , })) }
};
}
