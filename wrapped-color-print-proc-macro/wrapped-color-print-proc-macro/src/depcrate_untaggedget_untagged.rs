// Generated macro for get_untagged (function)
macro_rules! Depcrate_untaggedget_untagged {
() => {
// Module: crate::untagged
// Provides: {"get_untagged"}
// Dependencies: {}
# [doc = " Transforms a string literal by removing all its color tags."] pub fn get_untagged (input : TokenStream) -> Result < TokenStream2 , SpanError > { let args = parse_args (input) ? ; let format_string_token = get_format_string (args . first ()) ? ; let format_string = format_string_token . value () ; if args . len () > 1 { return Err (SpanError :: new (Error :: TooManyArgs , None)) ; } let format_nodes = parse_format_string (& format_string , & format_string_token) ? ; let mut format_string = String :: new () ; let mut color_context = Context :: new () ; for node in format_nodes { match node { Node :: Text (s) | Node :: Placeholder (s) => { format_string . push_str (s) ; } Node :: ColorTagGroup (tag_group) => { color_context . apply_tags (tag_group) ? ; } } } Ok (quote ! { # format_string }) }
};
}
