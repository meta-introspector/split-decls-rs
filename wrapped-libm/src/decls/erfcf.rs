macro_rules! erfcf {
    () => {
        # [doc = " Complementary error function (f32)"] # [doc = ""] # [doc = " Calculates the complementary probability."] # [doc = " Is `1 - erf(x)`. Is computed directly, so that you can use it to avoid"] # [doc = " the loss of precision that would result from subtracting"] # [doc = " large probabilities (on large `x`) from 1."] pub fn erfcf (x : f32) -> f32 { let r : f32 ; let s : f32 ; let z : f32 ; let y : f32 ; let mut ix : u32 ; let sign : usize ; ix = x . to_bits () ; sign = (ix >> 31) as usize ; ix &= 0x7fffffff ; if ix >= 0x7f800000 { return 2.0 * (sign as f32) + 1.0 / x ; } if ix < 0x3f580000 { if ix < 0x23800000 { return 1.0 - x ; } z = x * x ; r = PP0 + z * (PP1 + z * (PP2 + z * (PP3 + z * PP4))) ; s = 1.0 + z * (QQ1 + z * (QQ2 + z * (QQ3 + z * (QQ4 + z * QQ5)))) ; y = r / s ; if sign != 0 || ix < 0x3e800000 { return 1.0 - (x + x * y) ; } return 0.5 - (x - 0.5 + x * y) ; } if ix < 0x41e00000 { if sign != 0 { return 2.0 - erfc2 (ix , x) ; } else { return erfc2 (ix , x) ; } } let x1p_120 = f32 :: from_bits (0x03800000) ; if sign != 0 { 2.0 - x1p_120 } else { x1p_120 * x1p_120 } }
    };
}

erfcf!();