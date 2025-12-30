// Generated macro for is_non_whitespace (function)
macro_rules! Depcrate_deis_non_whitespace {
() => {
// Module: crate::de
// Provides: {"is_non_whitespace"}
// Dependencies: {}
# [doc = " A function to check whether the character is a whitespace (blank, new line, carriage return or tab)."] # [inline] const fn is_non_whitespace (ch : char) -> bool { ! matches ! (ch , ' ' | '\r' | '\n' | '\t') }
};
}
