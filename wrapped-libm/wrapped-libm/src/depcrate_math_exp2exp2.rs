// Generated macro for exp2 (function)
macro_rules! Depcrate_math_exp2exp2 {
() => {
// Module: crate::math::exp2
// Provides: {"exp2"}
// Dependencies: {}
# [doc = " Exponential, base 2 (f64)"] # [doc = ""] # [doc = " Calculate `2^x`, that is, 2 raised to the power `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn exp2 (mut x : f64) -> f64 { let redux = f64 :: from_bits (0x4338000000000000) / TBLSIZE as f64 ; let p1 = f64 :: from_bits (0x3fe62e42fefa39ef) ; let p2 = f64 :: from_bits (0x3fcebfbdff82c575) ; let p3 = f64 :: from_bits (0x3fac6b08d704a0a6) ; let p4 = f64 :: from_bits (0x3f83b2ab88f70400) ; let p5 = f64 :: from_bits (0x3f55d88003875c74) ; let x1p1023 = f64 :: from_bits (0x7fe0000000000000) ; let x1p52 = f64 :: from_bits (0x4330000000000000) ; let _0x1p_149 = f64 :: from_bits (0xb6a0000000000000) ; let ui = f64 :: to_bits (x) ; let ix = (ui >> 32) & 0x7fffffff ; if ix >= 0x408ff000 { if ix >= 0x40900000 && ui >> 63 == 0 { x *= x1p1023 ; return x ; } if ix >= 0x7ff00000 { return - 1.0 / x ; } if ui >> 63 != 0 { if x <= - 1075.0 || x - x1p52 + x1p52 != x { force_eval ! ((_0x1p_149 / x) as f32) ; } if x <= - 1075.0 { return 0.0 ; } } } else if ix < 0x3c900000 { return 1.0 + x ; } let ui = f64 :: to_bits (x + redux) ; let mut i0 = ui as u32 ; i0 = i0 . wrapping_add (TBLSIZE as u32 / 2) ; let ku = i0 / TBLSIZE as u32 * TBLSIZE as u32 ; let ki = div ! (ku as i32 , TBLSIZE as i32) ; i0 %= TBLSIZE as u32 ; let uf = f64 :: from_bits (ui) - redux ; let mut z = x - uf ; let t = f64 :: from_bits (i ! (TBL , 2 * i0 as usize)) ; z -= f64 :: from_bits (i ! (TBL , 2 * i0 as usize + 1)) ; let r = t + t * z * (p1 + z * (p2 + z * (p3 + z * (p4 + z * p5)))) ; scalbn (r , ki) }
};
}
