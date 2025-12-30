// Generated macro for get_description_height (function)
macro_rules! Depcrateget_description_height {
() => {
// Module: crate
// Provides: {"get_description_height"}
// Dependencies: {}
# [expect (clippy :: cast_possible_truncation)] fn get_description_height (s : & str) -> u16 { if s . is_empty () { 0 } else { s . split ('\n') . count () as u16 } }
};
}
