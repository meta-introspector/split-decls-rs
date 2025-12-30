// Generated macro for crc128 (function)
macro_rules! Depcrate_utilcrc128 {
() => {
// Module: crate::util
// Provides: {"crc128"}
// Dependencies: {}
pub (crate) const fn crc128 (poly : u128 , reflect : bool , mut value : u128) -> u128 { if reflect { let mut i = 0 ; while i < 8 { value = (value >> 1) ^ ((value & 1) * poly) ; i += 1 ; } } else { value <<= 120 ; let mut i = 0 ; while i < 8 { value = (value << 1) ^ (((value >> 127) & 1) * poly) ; i += 1 ; } } value }
};
}
