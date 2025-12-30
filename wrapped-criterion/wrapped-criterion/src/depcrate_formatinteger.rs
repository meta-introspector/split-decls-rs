// Generated macro for integer (function)
macro_rules! Depcrate_formatinteger {
() => {
// Module: crate::format
// Provides: {"integer"}
// Dependencies: {}
# [doc = " Format a value as an integer, including thousands-separators."] pub fn integer (n : f64) -> String { thousands_sep (n as u64 , ',') }
};
}
