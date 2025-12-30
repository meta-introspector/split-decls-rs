// Generated macro for get_cstr (function)
macro_rules! Depcrate_ansiget_cstr {
() => {
// Module: crate::ansi
// Provides: {"get_cstr"}
// Dependencies: {}
# [doc = " Transforms a string literal by parsing its color tags."] pub fn get_cstr (input : TokenStream) -> Result < TokenStream2 , SpanError > { let args = parse_args (input) ? ; let format_string_token = get_format_string (args . first ()) ? ; let format_string = format_string_token . value () ; if args . len () > 1 { return Err (SpanError :: new (Error :: TooManyArgs , None)) ; } let format_nodes = parse_format_string (& format_string , & format_string_token) ? ; get_format_string_from_nodes (format_nodes) }
};
}
