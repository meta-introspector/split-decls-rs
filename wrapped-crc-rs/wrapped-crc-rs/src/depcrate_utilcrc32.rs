// Generated macro for crc32 (function)
macro_rules! Depcrate_utilcrc32 {
() => {
// Module: crate::util
// Provides: {"crc32"}
// Dependencies: {}
pub (crate) const fn crc32 (poly : u32 , reflect : bool , mut value : u32) -> u32 { if reflect { let mut i = 0 ; while i < 8 { value = (value >> 1) ^ ((value & 1) * poly) ; i += 1 ; } } else { value <<= 24 ; let mut i = 0 ; while i < 8 { value = (value << 1) ^ (((value >> 31) & 1) * poly) ; i += 1 ; } } value }
};
}
