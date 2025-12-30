// Generated macro for get_format_string_from_nodes (function)
macro_rules! Depcrate_ansiget_format_string_from_nodes {
() => {
// Module: crate::ansi
// Provides: {"get_format_string_from_nodes"}
// Dependencies: {}
# [doc = " Generates a new format string with the color tags replaced by the right ANSI codes."] fn get_format_string_from_nodes (nodes : Vec < Node >) -> Result < TokenStream2 , SpanError > { let mut format_string = String :: new () ; let mut color_context = Context :: new () ; for node in nodes { match node { Node :: Text (s) | Node :: Placeholder (s) => { format_string . push_str (s) ; } Node :: ColorTagGroup (tag_group) => { let ansi_string = color_context . ansi_apply_tags (tag_group) ? ; format_string . push_str (& ansi_string) ; } } } Ok (quote ! { # format_string }) }
};
}
