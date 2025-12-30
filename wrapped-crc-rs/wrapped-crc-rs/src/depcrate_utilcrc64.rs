// Generated macro for crc64 (function)
macro_rules! Depcrate_utilcrc64 {
() => {
// Module: crate::util
// Provides: {"crc64"}
// Dependencies: {}
pub (crate) const fn crc64 (poly : u64 , reflect : bool , mut value : u64) -> u64 { if reflect { let mut i = 0 ; while i < 8 { value = (value >> 1) ^ ((value & 1) * poly) ; i += 1 ; } } else { value <<= 56 ; let mut i = 0 ; while i < 8 { value = (value << 1) ^ (((value >> 63) & 1) * poly) ; i += 1 ; } } value }
};
}
