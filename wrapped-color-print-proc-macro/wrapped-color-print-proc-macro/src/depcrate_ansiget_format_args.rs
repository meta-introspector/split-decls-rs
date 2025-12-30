// Generated macro for get_format_args (function)
macro_rules! Depcrate_ansiget_format_args {
() => {
// Module: crate::ansi
// Provides: {"get_format_args"}
// Dependencies: {}
# [doc = " Common code shared between the public macros, ANSI implementation."] pub fn get_format_args (input : TokenStream) -> Result < TokenStream2 , SpanError > { let (format_string_token , args) = get_args_and_format_string (input) ? ; let format_string = format_string_token . value () ; let format_nodes = parse_format_string (& format_string , & format_string_token) ? ; let final_format_string = get_format_string_from_nodes (format_nodes) ? ; let args = args . iter () . map (| arg | arg . to_token_stream ()) . skip (1) ; let final_args = std :: iter :: once (final_format_string) . chain (args) ; Ok (quote ! { # (# final_args) ,* }) }
};
}
