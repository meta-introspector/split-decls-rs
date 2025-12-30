// Generated macro for contains_unhygienic_crate_reference (function)
macro_rules! Depcrate_crate_in_macro_defcontains_unhygienic_crate_reference {
() => {
// Module: crate::crate_in_macro_def
// Provides: {"contains_unhygienic_crate_reference"}
// Dependencies: {}
fn contains_unhygienic_crate_reference (tts : & TokenStream) -> Option < Span > { let mut prev_is_dollar = false ; let mut iter = tts . iter () ; while let Some (curr) = iter . next () { if ! prev_is_dollar && let Some (span) = is_crate_keyword (curr) && let Some (next) = iter . peek () && is_token (next , & TokenKind :: PathSep) { return Some (span) ; } if let TokenTree :: Delimited (.. , tts) = & curr { let span = contains_unhygienic_crate_reference (tts) ; if span . is_some () { return span ; } } prev_is_dollar = is_token (curr , & TokenKind :: Dollar) ; } None }
};
}
