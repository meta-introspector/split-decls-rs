// Generated macro for log1pf (function)
macro_rules! Depcrate_math_log1pflog1pf {
() => {
// Module: crate::math::log1pf
// Provides: {"log1pf"}
// Dependencies: {}
# [doc = " The natural logarithm of 1+`x` (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn log1pf (x : f32) -> f32 { let mut ui : u32 = x . to_bits () ; let hfsq : f32 ; let mut f : f32 = 0. ; let mut c : f32 = 0. ; let s : f32 ; let z : f32 ; let r : f32 ; let w : f32 ; let t1 : f32 ; let t2 : f32 ; let dk : f32 ; let ix : u32 ; let mut iu : u32 ; let mut k : i32 ; ix = ui ; k = 1 ; if ix < 0x3ed413d0 || (ix >> 31) > 0 { if ix >= 0xbf800000 { if x == - 1. { return x / 0.0 ; } return (x - x) / 0.0 ; } if ix << 1 < 0x33800000 << 1 { if (ix & 0x7f800000) == 0 { force_eval ! (x * x) ; } return x ; } if ix <= 0xbe95f619 { k = 0 ; c = 0. ; f = x ; } } else if ix >= 0x7f800000 { return x ; } if k > 0 { ui = (1. + x) . to_bits () ; iu = ui ; iu += 0x3f800000 - 0x3f3504f3 ; k = (iu >> 23) as i32 - 0x7f ; if k < 25 { c = if k >= 2 { 1. - (f32 :: from_bits (ui) - x) } else { x - (f32 :: from_bits (ui) - 1.) } ; c /= f32 :: from_bits (ui) ; } else { c = 0. ; } iu = (iu & 0x007fffff) + 0x3f3504f3 ; ui = iu ; f = f32 :: from_bits (ui) - 1. ; } s = f / (2.0 + f) ; z = s * s ; w = z * z ; t1 = w * (LG2 + w * LG4) ; t2 = z * (LG1 + w * LG3) ; r = t2 + t1 ; hfsq = 0.5 * f * f ; dk = k as f32 ; s * (hfsq + r) + (dk * LN2_LO + c) - hfsq + f + dk * LN2_HI }
};
}
