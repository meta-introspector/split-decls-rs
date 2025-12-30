// Generated macro for parse_input (function)
macro_rules! Depcrate_parseparse_input {
() => {
// Module: crate::parse
// Provides: {"parse_input"}
// Dependencies: {}
pub (crate) fn parse_input (input : TokenStream ,) -> Result < (Vec < Attribute > , Vec < TokenTree > , TokenTree) > { let mut input = input . into_iter () . peekable () ; let mut attrs = Vec :: new () ; while let Some (attr) = parse_next_attr (& mut input) ? { attrs . push (attr) ; } let sig = parse_signature (& mut input) ; let body = input . next () . ok_or_else (| | { Error :: new (Span :: call_site () , "`#[proc_macro_error]` can be applied only to functions" . to_string () ,) }) ? ; Ok ((attrs , sig , body)) }
};
}
