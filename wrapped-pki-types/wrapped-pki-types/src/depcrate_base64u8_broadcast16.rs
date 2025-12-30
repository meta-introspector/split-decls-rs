// Generated macro for u8_broadcast16 (function)
macro_rules! Depcrate_base64u8_broadcast16 {
() => {
// Module: crate::base64
// Provides: {"u8_broadcast16"}
// Dependencies: {}
# [doc = " Broadcasts the top bit of `x`"] # [doc = ""] # [doc = " In other words, if the top bit of `x` is set,"] # [doc = " returns 0xff else 0x00."] const fn u8_broadcast16 (x : u16) -> u8 { let msb = x >> 15 ; 0u8 . wrapping_sub (msb as u8) }
};
}
