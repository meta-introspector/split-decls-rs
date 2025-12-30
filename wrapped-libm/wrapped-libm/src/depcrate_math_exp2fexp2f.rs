// Generated macro for exp2f (function)
macro_rules! Depcrate_math_exp2fexp2f {
() => {
// Module: crate::math::exp2f
// Provides: {"exp2f"}
// Dependencies: {}
# [doc = " Exponential, base 2 (f32)"] # [doc = ""] # [doc = " Calculate `2^x`, that is, 2 raised to the power `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn exp2f (mut x : f32) -> f32 { let redux = f32 :: from_bits (0x4b400000) / TBLSIZE as f32 ; let p1 = f32 :: from_bits (0x3f317218) ; let p2 = f32 :: from_bits (0x3e75fdf0) ; let p3 = f32 :: from_bits (0x3d6359a4) ; let p4 = f32 :: from_bits (0x3c1d964e) ; let x1p127 = f32 :: from_bits (0x7f000000) ; let ui = f32 :: to_bits (x) ; let ix = ui & 0x7fffffff ; if ix > 0x42fc0000 { if ix > 0x7f800000 { return x ; } if (0x43000000 .. 0x80000000) . contains (& ui) { x *= x1p127 ; return x ; } if ui >= 0x80000000 { if ui >= 0xc3160000 || (ui & 0x0000ffff != 0) { force_eval ! (f32 :: from_bits (0x80000001) / x) ; } if ui >= 0xc3160000 { return 0.0 ; } } } else if ix <= 0x33000000 { return 1.0 + x ; } let ui = f32 :: to_bits (x + redux) ; let mut i0 = ui ; i0 += TBLSIZE as u32 / 2 ; let k = i0 / TBLSIZE as u32 ; let ukf = f64 :: from_bits (((0x3ff + k) as u64) << 52) ; i0 &= TBLSIZE as u32 - 1 ; let mut uf = f32 :: from_bits (ui) ; uf -= redux ; let z : f64 = (x - uf) as f64 ; let r : f64 = f64 :: from_bits (i ! (EXP2FT , i0 as usize)) ; let t : f64 = r * z ; let r : f64 = r + t * (p1 as f64 + z * p2 as f64) + t * (z * z) * (p3 as f64 + z * p4 as f64) ; (r * ukf) as f32 }
};
}
