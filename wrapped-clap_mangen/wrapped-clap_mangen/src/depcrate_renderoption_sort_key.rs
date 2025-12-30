// Generated macro for option_sort_key (function)
macro_rules! Depcrate_renderoption_sort_key {
() => {
// Module: crate::render
// Provides: {"option_sort_key"}
// Dependencies: {}
# [doc = " Note that this function is duplicated from `clap::builder`"] fn option_sort_key (arg : & Arg) -> (usize , String) { let key = if let Some (x) = arg . get_short () { let mut s = x . to_ascii_lowercase () . to_string () ; s . push (if x . is_ascii_lowercase () { '0' } else { '1' }) ; s } else if let Some (x) = arg . get_long () { x . to_string () } else { let mut s = '{' . to_string () ; s . push_str (arg . get_id () . as_str ()) ; s } ; (arg . get_display_order () , key) }
};
}
