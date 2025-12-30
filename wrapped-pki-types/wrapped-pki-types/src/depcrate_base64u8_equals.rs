// Generated macro for u8_equals (function)
macro_rules! Depcrate_base64u8_equals {
() => {
// Module: crate::base64
// Provides: {"u8_equals"}
// Dependencies: {}
# [doc = " Returns 0xff if a == b, 0 otherwise."] const fn u8_equals (a : u8 , b : u8) -> u8 { let diff = a ^ b ; u8_nonzero (diff) }
};
}
