// Generated macro for gf_multiply (function)
macro_rules! Depcrate_utilsgf_multiply {
() => {
// Module: crate::utils
// Provides: {"gf_multiply"}
// Dependencies: {}
const fn gf_multiply (x : u8 , y : u8) -> u8 { const REDUCTION_POLYNOMIAL : u16 = 0x011d ; let mut x = x ; let mut y = y ; let mut r = 0u8 ; let mut i = 0 ; while i < u8 :: BITS { if y & 1 == 1 { r ^= x ; } let hbit = (x & 0x80) >> 7 ; x <<= 1 ; if hbit == 1 { x ^= REDUCTION_POLYNOMIAL as u8 ; } y >>= 1 ; i += 1 ; } r }
};
}
