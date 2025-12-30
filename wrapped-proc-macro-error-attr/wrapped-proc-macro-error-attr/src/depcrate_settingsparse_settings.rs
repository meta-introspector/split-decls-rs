// Generated macro for parse_settings (function)
macro_rules! Depcrate_settingsparse_settings {
() => {
// Module: crate::settings
// Provides: {"parse_settings"}
// Dependencies: {}
pub (crate) fn parse_settings (input : TokenStream) -> Result < Settings > { let mut input = input . into_iter () ; let mut res = Settings (Vec :: new ()) ; loop { match input . next () { Some (TokenTree :: Ident (ident)) => { res . 0 . push (ident_to_setting (ident) ?) ; } None => return Ok (res) , other => { let span = other . map_or (Span :: call_site () , | tt | tt . span ()) ; return Err (Error :: new (span , "expected identifier" . to_string ())) ; } } match input . next () { Some (TokenTree :: Punct (ref punct)) if punct . as_char () == ',' => { } None => return Ok (res) , other => { let span = other . map_or (Span :: call_site () , | tt | tt . span ()) ; return Err (Error :: new (span , "expected `,`" . to_string ())) ; } } } }
};
}
