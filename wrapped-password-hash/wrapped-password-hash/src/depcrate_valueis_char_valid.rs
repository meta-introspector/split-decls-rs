// Generated macro for is_char_valid (function)
macro_rules! Depcrate_valueis_char_valid {
() => {
// Module: crate::value
// Provides: {"is_char_valid"}
// Dependencies: {}
# [doc = " Ensure the given ASCII character (i.e. byte) is allowed in a [`Value`]."] fn is_char_valid (c : char) -> bool { matches ! (c , 'A' ..= 'Z' | 'a' ..='z' | '0' ..='9' | '/' | '+' | '.' | '-') }
};
}
