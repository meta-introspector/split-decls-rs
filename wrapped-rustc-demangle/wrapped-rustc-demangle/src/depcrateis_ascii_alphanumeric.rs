// Generated macro for is_ascii_alphanumeric (function)
macro_rules! Depcrateis_ascii_alphanumeric {
() => {
// Module: crate
// Provides: {"is_ascii_alphanumeric"}
// Dependencies: {}
fn is_ascii_alphanumeric (c : char) -> bool { match c { '\u{0041}' ..= '\u{005A}' | '\u{0061}' ..= '\u{007A}' | '\u{0030}' ..= '\u{0039}' => true , _ => false , } }
};
}
