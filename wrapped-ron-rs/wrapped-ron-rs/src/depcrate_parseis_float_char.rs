// Generated macro for is_float_char (function)
macro_rules! Depcrate_parseis_float_char {
() => {
// Module: crate::parse
// Provides: {"is_float_char"}
// Dependencies: {}
const fn is_float_char (c : char) -> bool { c . is_ascii_digit () || matches ! (c , 'e' | 'E' | '.' | '+' | '-' | '_') }
};
}
