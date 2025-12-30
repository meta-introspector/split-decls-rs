// Generated macro for parse_index (function)
macro_rules! Depcrate_valueparse_index {
() => {
// Module: crate::value
// Provides: {"parse_index"}
// Dependencies: {}
fn parse_index (s : & str) -> Option < usize > { if s . starts_with ('+') || (s . starts_with ('0') && s . len () != 1) { return None ; } s . parse () . ok () }
};
}
