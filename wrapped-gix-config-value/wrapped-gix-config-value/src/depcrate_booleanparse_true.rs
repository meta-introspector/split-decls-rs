// Generated macro for parse_true (function)
macro_rules! Depcrate_booleanparse_true {
() => {
// Module: crate::boolean
// Provides: {"parse_true"}
// Dependencies: {}
fn parse_true (value : & BStr) -> bool { value . eq_ignore_ascii_case (b"yes") || value . eq_ignore_ascii_case (b"on") || value . eq_ignore_ascii_case (b"true") }
};
}
