// Generated macro for chars_eq (function)
macro_rules! Depcratechars_eq {
() => {
// Module: crate
// Provides: {"chars_eq"}
// Dependencies: {}
# [doc = " A helper function to determine if two chars are (possibly case-insensitively) equal."] fn chars_eq (a : char , b : char , case_sensitive : bool) -> bool { if cfg ! (windows) && path :: is_separator (a) && path :: is_separator (b) { true } else if ! case_sensitive && a . is_ascii () && b . is_ascii () { a . eq_ignore_ascii_case (& b) } else { a == b } }
};
}
