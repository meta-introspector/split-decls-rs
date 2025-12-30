// Generated macro for trim_trailing_spaces (function)
macro_rules! Depcrate_attrtrim_trailing_spaces {
() => {
// Module: crate::attr
// Provides: {"trim_trailing_spaces"}
// Dependencies: {}
fn trim_trailing_spaces (doc : & mut String) { doc . truncate (doc . trim_end_matches (' ') . len ()) ; }
};
}
