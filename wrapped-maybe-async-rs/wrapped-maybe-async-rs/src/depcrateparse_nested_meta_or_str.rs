// Generated macro for parse_nested_meta_or_str (function)
macro_rules! Depcrateparse_nested_meta_or_str {
() => {
// Module: crate
// Provides: {"parse_nested_meta_or_str"}
// Dependencies: {}
fn parse_nested_meta_or_str (input : ParseStream) -> Result < TokenStream2 > { if let Some (s) = input . parse :: < Option < LitStr > > () ? { let tokens = s . value () . parse () ? ; Ok (tokens) } else { let meta : Meta = input . parse () ? ; Ok (quote ! (# meta)) } }
};
}
