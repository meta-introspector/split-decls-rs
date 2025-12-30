// Generated macro for grapheme_is_uppercase (function)
macro_rules! Depcrate_boundarygrapheme_is_uppercase {
() => {
// Module: crate::boundary
// Provides: {"grapheme_is_uppercase"}
// Dependencies: {}
fn grapheme_is_uppercase (c : & & str) -> bool { c . to_uppercase () != c . to_lowercase () && * c == c . to_uppercase () }
};
}
