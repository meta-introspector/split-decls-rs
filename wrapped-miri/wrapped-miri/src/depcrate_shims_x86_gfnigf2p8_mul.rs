// Generated macro for gf2p8_mul (function)
macro_rules! Depcrate_shims_x86_gfnigf2p8_mul {
() => {
// Module: crate::shims::x86::gfni
// Provides: {"gf2p8_mul"}
// Dependencies: {}
# [doc = " Multiplies packed 8-bit integers in `left` and `right` in the finite field GF(2^8)"] # [doc = " and store the results in `dst`. The field GF(2^8) is represented in"] # [doc = " polynomial representation with the reduction polynomial x^8 + x^4 + x^3 + x + 1."] # [doc = " See <https://www.corsix.org/content/galois-field-instructions-2021-cpus> for details."] # [expect (clippy :: as_conversions)] const fn gf2p8_mul (left : u8 , right : u8) -> u8 { const POLYNOMIAL : u32 = 0x11b ; let left = left as u32 ; let right = right as u32 ; let mut result = 0u32 ; let mut i = 0u32 ; while i < 8 { if left & (1 << i) != 0 { result ^= right << i ; } i = i . wrapping_add (1) ; } let mut i = 14u32 ; while i >= 8 { if result & (1 << i) != 0 { result ^= POLYNOMIAL << i . wrapping_sub (8) ; } i = i . wrapping_sub (1) ; } result as u8 }
};
}
