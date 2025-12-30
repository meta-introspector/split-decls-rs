// Generated macro for u8_broadcast8 (function)
macro_rules! Depcrate_base64u8_broadcast8 {
() => {
// Module: crate::base64
// Provides: {"u8_broadcast8"}
// Dependencies: {}
# [doc = " Broadcasts the top bit of `x`"] # [doc = ""] # [doc = " In other words, if the top bit of `x` is set,"] # [doc = " returns 0xff else 0x00."] const fn u8_broadcast8 (x : u8) -> u8 { let msb = x >> 7 ; 0u8 . wrapping_sub (msb) }
};
}
