// Generated macro for last_char_of_first_line (function)
macro_rules! Depcratelast_char_of_first_line {
() => {
// Module: crate
// Provides: {"last_char_of_first_line"}
// Dependencies: {}
fn last_char_of_first_line (text : & str) -> Option < char > { text . lines () . next () ? . chars () . last () }
};
}
