// Generated macro for is_whitespace (function)
macro_rules! Depcrate_utilsis_whitespace {
() => {
// Module: crate::utils
// Provides: {"is_whitespace"}
// Dependencies: {}
# [doc = " A function to check whether the byte is a whitespace (blank, new line, carriage return or tab)."] # [inline] pub const fn is_whitespace (b : u8) -> bool { matches ! (b , b' ' | b'\r' | b'\n' | b'\t') }
};
}
