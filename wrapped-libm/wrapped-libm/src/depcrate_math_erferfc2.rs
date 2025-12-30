// Generated macro for erfc2 (function)
macro_rules! Depcrate_math_erferfc2 {
() => {
// Module: crate::math::erf
// Provides: {"erfc2"}
// Dependencies: {}
fn erfc2 (ix : u32 , mut x : f64) -> f64 { let s : f64 ; let r : f64 ; let big_s : f64 ; let z : f64 ; if ix < 0x3ff40000 { return erfc1 (x) ; } x = fabs (x) ; s = 1.0 / (x * x) ; if ix < 0x4006db6d { r = RA0 + s * (RA1 + s * (RA2 + s * (RA3 + s * (RA4 + s * (RA5 + s * (RA6 + s * RA7)))))) ; big_s = 1.0 + s * (SA1 + s * (SA2 + s * (SA3 + s * (SA4 + s * (SA5 + s * (SA6 + s * (SA7 + s * SA8))))))) ; } else { r = RB0 + s * (RB1 + s * (RB2 + s * (RB3 + s * (RB4 + s * (RB5 + s * RB6))))) ; big_s = 1.0 + s * (SB1 + s * (SB2 + s * (SB3 + s * (SB4 + s * (SB5 + s * (SB6 + s * SB7)))))) ; } z = with_set_low_word (x , 0) ; exp (- z * z - 0.5625) * exp ((z - x) * (z + x) + r / big_s) / x }
};
}
