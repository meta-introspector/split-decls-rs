// Generated macro for is_valid_java_cesu8 (function)
macro_rules! Depcrateis_valid_java_cesu8 {
() => {
// Module: crate
// Provides: {"is_valid_java_cesu8"}
// Dependencies: {}
# [doc = " Check whether a Rust string contains valid Java's modified UTF-8 data."] pub fn is_valid_java_cesu8 (text : & str) -> bool { ! text . contains ('\0') && is_valid_cesu8 (text) }
};
}
