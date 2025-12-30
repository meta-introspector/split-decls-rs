// Generated macro for path_push (function)
macro_rules! Depcrate_linepath_push {
() => {
// Module: crate::line
// Provides: {"path_push"}
// Dependencies: {}
fn path_push (path : & mut String , p : & str) { if has_forward_slash_root (p) || has_backward_slash_root (p) { * path = p . to_string () ; } else { let dir_separator = if has_backward_slash_root (path . as_str ()) { '\\' } else { '/' } ; if ! path . is_empty () && ! path . ends_with (dir_separator) { path . push (dir_separator) ; } * path += p ; } }
};
}
