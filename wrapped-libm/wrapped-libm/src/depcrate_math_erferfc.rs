// Generated macro for erfc (function)
macro_rules! Depcrate_math_erferfc {
() => {
// Module: crate::math::erf
// Provides: {"erfc"}
// Dependencies: {}
# [doc = " Complementary error function (f64)"] # [doc = ""] # [doc = " Calculates the complementary probability."] # [doc = " Is `1 - erf(x)`. Is computed directly, so that you can use it to avoid"] # [doc = " the loss of precision that would result from subtracting"] # [doc = " large probabilities (on large `x`) from 1."] pub fn erfc (x : f64) -> f64 { let r : f64 ; let s : f64 ; let z : f64 ; let y : f64 ; let mut ix : u32 ; let sign : usize ; ix = get_high_word (x) ; sign = (ix >> 31) as usize ; ix &= 0x7fffffff ; if ix >= 0x7ff00000 { return 2.0 * (sign as f64) + 1.0 / x ; } if ix < 0x3feb0000 { if ix < 0x3c700000 { return 1.0 - x ; } z = x * x ; r = PP0 + z * (PP1 + z * (PP2 + z * (PP3 + z * PP4))) ; s = 1.0 + z * (QQ1 + z * (QQ2 + z * (QQ3 + z * (QQ4 + z * QQ5)))) ; y = r / s ; if sign != 0 || ix < 0x3fd00000 { return 1.0 - (x + x * y) ; } return 0.5 - (x - 0.5 + x * y) ; } if ix < 0x403c0000 { if sign != 0 { return 2.0 - erfc2 (ix , x) ; } else { return erfc2 (ix , x) ; } } let x1p_1022 = f64 :: from_bits (0x0010000000000000) ; if sign != 0 { 2.0 - x1p_1022 } else { x1p_1022 * x1p_1022 } }
};
}
