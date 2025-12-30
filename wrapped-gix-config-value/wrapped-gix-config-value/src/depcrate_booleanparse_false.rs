// Generated macro for parse_false (function)
macro_rules! Depcrate_booleanparse_false {
() => {
// Module: crate::boolean
// Provides: {"parse_false"}
// Dependencies: {}
fn parse_false (value : & BStr) -> bool { value . eq_ignore_ascii_case (b"no") || value . eq_ignore_ascii_case (b"off") || value . eq_ignore_ascii_case (b"false") || value . is_empty () }
};
}
