// Generated macro for parse_custom_message (function)
macro_rules! Depcrate_assertparse_custom_message {
() => {
// Module: crate::assert
// Provides: {"parse_custom_message"}
// Dependencies: {}
fn parse_custom_message (parser : & mut Parser < '_ >) -> Option < TokenStream > { let ts = parser . parse_tokens () ; if ! ts . is_empty () { Some (ts) } else { None } }
};
}
