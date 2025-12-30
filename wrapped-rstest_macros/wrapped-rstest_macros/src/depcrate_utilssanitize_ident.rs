// Generated macro for sanitize_ident (function)
macro_rules! Depcrate_utilssanitize_ident {
() => {
// Module: crate::utils
// Provides: {"sanitize_ident"}
// Dependencies: {}
pub (crate) fn sanitize_ident (name : & str) -> String { name . chars () . filter (| c | ! c . is_whitespace ()) . map (| c | match c { '"' | '\'' => "__" . to_owned () , ':' | '(' | ')' | '{' | '}' | '[' | ']' | ',' | '.' | '*' | '+' | '/' | '-' | '%' | '^' | '!' | '&' | '|' => "_" . to_owned () , _ => c . to_string () , }) . collect :: < String > () . chars () . filter (| & c | is_xid_continue (c)) . collect () }
};
}
