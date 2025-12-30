// Generated macro for crc8 (function)
macro_rules! Depcrate_utilcrc8 {
() => {
// Module: crate::util
// Provides: {"crc8"}
// Dependencies: {}
pub (crate) const fn crc8 (poly : u8 , reflect : bool , mut value : u8) -> u8 { let mut i = 0 ; if reflect { while i < 8 { value = (value >> 1) ^ ((value & 1) * poly) ; i += 1 ; } } else { while i < 8 { value = (value << 1) ^ (((value >> 7) & 1) * poly) ; i += 1 ; } } value }
};
}
