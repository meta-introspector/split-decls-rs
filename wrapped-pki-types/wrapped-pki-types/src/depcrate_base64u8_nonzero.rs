// Generated macro for u8_nonzero (function)
macro_rules! Depcrate_base64u8_nonzero {
() => {
// Module: crate::base64
// Provides: {"u8_nonzero"}
// Dependencies: {}
# [doc = " Returns 0xff if a != 0, 0 otherwise."] const fn u8_nonzero (x : u8) -> u8 { u8_broadcast8 (! x & x . wrapping_sub (1)) }
};
}
