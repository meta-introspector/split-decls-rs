// Generated macro for erfc2 (function)
macro_rules! Depcrate_math_erfferfc2 {
() => {
// Module: crate::math::erff
// Provides: {"erfc2"}
// Dependencies: {}
fn erfc2 (mut ix : u32 , mut x : f32) -> f32 { let s : f32 ; let r : f32 ; let big_s : f32 ; let z : f32 ; if ix < 0x3fa00000 { return erfc1 (x) ; } x = fabsf (x) ; s = 1.0 / (x * x) ; if ix < 0x4036db6d { r = RA0 + s * (RA1 + s * (RA2 + s * (RA3 + s * (RA4 + s * (RA5 + s * (RA6 + s * RA7)))))) ; big_s = 1.0 + s * (SA1 + s * (SA2 + s * (SA3 + s * (SA4 + s * (SA5 + s * (SA6 + s * (SA7 + s * SA8))))))) ; } else { r = RB0 + s * (RB1 + s * (RB2 + s * (RB3 + s * (RB4 + s * (RB5 + s * RB6))))) ; big_s = 1.0 + s * (SB1 + s * (SB2 + s * (SB3 + s * (SB4 + s * (SB5 + s * (SB6 + s * SB7)))))) ; } ix = x . to_bits () ; z = f32 :: from_bits (ix & 0xffffe000) ; expf (- z * z - 0.5625) * expf ((z - x) * (z + x) + r / big_s) / x }
};
}
