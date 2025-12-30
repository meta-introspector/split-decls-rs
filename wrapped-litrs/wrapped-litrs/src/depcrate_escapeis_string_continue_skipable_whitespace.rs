// Generated macro for is_string_continue_skipable_whitespace (function)
macro_rules! Depcrate_escapeis_string_continue_skipable_whitespace {
() => {
// Module: crate::escape
// Provides: {"is_string_continue_skipable_whitespace"}
// Dependencies: {}
# [doc = " Checks whether the character is skipped after a string continue start"] # [doc = " (unescaped backlash followed by `\\n`)."] fn is_string_continue_skipable_whitespace (b : u8) -> bool { b == b' ' || b == b'\t' || b == b'\n' }
};
}
