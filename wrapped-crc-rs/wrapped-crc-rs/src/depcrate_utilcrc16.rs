// Generated macro for crc16 (function)
macro_rules! Depcrate_utilcrc16 {
() => {
// Module: crate::util
// Provides: {"crc16"}
// Dependencies: {}
pub (crate) const fn crc16 (poly : u16 , reflect : bool , mut value : u16) -> u16 { if reflect { let mut i = 0 ; while i < 8 { value = (value >> 1) ^ ((value & 1) * poly) ; i += 1 ; } } else { value <<= 8 ; let mut i = 0 ; while i < 8 { value = (value << 1) ^ (((value >> 15) & 1) * poly) ; i += 1 ; } } value }
};
}
