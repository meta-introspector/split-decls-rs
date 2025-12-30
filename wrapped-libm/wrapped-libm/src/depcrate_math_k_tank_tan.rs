// Generated macro for k_tan (function)
macro_rules! Depcrate_math_k_tank_tan {
() => {
// Module: crate::math::k_tan
// Provides: {"k_tan"}
// Dependencies: {}
# [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_tan (mut x : f64 , mut y : f64 , odd : i32) -> f64 { let hx = (f64 :: to_bits (x) >> 32) as u32 ; let big = (hx & 0x7fffffff) >= 0x3FE59428 ; if big { let sign = hx >> 31 ; if sign != 0 { x = - x ; y = - y ; } x = (PIO4 - x) + (PIO4_LO - y) ; y = 0.0 ; } let z = x * x ; let w = z * z ; let r = T [1] + w * (T [3] + w * (T [5] + w * (T [7] + w * (T [9] + w * T [11])))) ; let v = z * (T [2] + w * (T [4] + w * (T [6] + w * (T [8] + w * (T [10] + w * T [12]))))) ; let s = z * x ; let r = y + z * (s * (r + v) + y) + s * T [0] ; let w = x + r ; if big { let sign = hx >> 31 ; let s = 1.0 - 2.0 * odd as f64 ; let v = s - 2.0 * (x + (r - w * w / (w + s))) ; return if sign != 0 { - v } else { v } ; } if odd == 0 { return w ; } let w0 = zero_low_word (w) ; let v = r - (w0 - x) ; let a = - 1.0 / w ; let a0 = zero_low_word (a) ; a0 + a * (1.0 + a0 * w0 + a0 * v) }
};
}
