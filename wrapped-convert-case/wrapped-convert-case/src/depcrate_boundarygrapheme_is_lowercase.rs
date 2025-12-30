// Generated macro for grapheme_is_lowercase (function)
macro_rules! Depcrate_boundarygrapheme_is_lowercase {
() => {
// Module: crate::boundary
// Provides: {"grapheme_is_lowercase"}
// Dependencies: {}
fn grapheme_is_lowercase (c : & & str) -> bool { c . to_uppercase () != c . to_lowercase () && * c == c . to_lowercase () }
};
}
