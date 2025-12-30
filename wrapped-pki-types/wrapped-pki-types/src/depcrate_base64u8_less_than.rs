// Generated macro for u8_less_than (function)
macro_rules! Depcrate_base64u8_less_than {
() => {
// Module: crate::base64
// Provides: {"u8_less_than"}
// Dependencies: {}
# [doc = " Returns 0xff if a < b, 0 otherwise."] fn u8_less_than (a : u8 , b : u8) -> u8 { let a = u16 :: from (a) ; let b = u16 :: from (b) ; u8_broadcast16 (a . wrapping_sub (b)) }
};
}
