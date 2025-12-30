// Generated macro for erff (function)
macro_rules! Depcrate_math_erfferff {
() => {
// Module: crate::math::erff
// Provides: {"erff"}
// Dependencies: {}
# [doc = " Error function (f32)"] # [doc = ""] # [doc = " Calculates an approximation to the “error function”, which estimates"] # [doc = " the probability that an observation will fall within x standard"] # [doc = " deviations of the mean (assuming a normal distribution)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn erff (x : f32) -> f32 { let r : f32 ; let s : f32 ; let z : f32 ; let y : f32 ; let mut ix : u32 ; let sign : usize ; ix = x . to_bits () ; sign = (ix >> 31) as usize ; ix &= 0x7fffffff ; if ix >= 0x7f800000 { return 1.0 - 2.0 * (sign as f32) + 1.0 / x ; } if ix < 0x3f580000 { if ix < 0x31800000 { return 0.125 * (8.0 * x + EFX8 * x) ; } z = x * x ; r = PP0 + z * (PP1 + z * (PP2 + z * (PP3 + z * PP4))) ; s = 1.0 + z * (QQ1 + z * (QQ2 + z * (QQ3 + z * (QQ4 + z * QQ5)))) ; y = r / s ; return x + x * y ; } if ix < 0x40c00000 { y = 1.0 - erfc2 (ix , x) ; } else { let x1p_120 = f32 :: from_bits (0x03800000) ; y = 1.0 - x1p_120 ; } if sign != 0 { - y } else { y } }
};
}
