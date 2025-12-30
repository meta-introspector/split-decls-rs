// Generated macro for grapheme_is_digit (function)
macro_rules! Depcrate_boundarygrapheme_is_digit {
() => {
// Module: crate::boundary
// Provides: {"grapheme_is_digit"}
// Dependencies: {}
fn grapheme_is_digit (c : & & str) -> bool { c . chars () . all (| c | c . is_ascii_digit ()) }
};
}
