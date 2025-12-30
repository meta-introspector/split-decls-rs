// Generated macro for rem_pio2f (function)
macro_rules! Depcrate_math_rem_pio2frem_pio2f {
() => {
// Module: crate::math::rem_pio2f
// Provides: {"rem_pio2f"}
// Dependencies: {}
# [doc = " Return the remainder of x rem pi/2 in *y"] # [doc = ""] # [doc = " use double precision for everything except passing x"] # [doc = " use __rem_pio2_large() for large x"] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn rem_pio2f (x : f32) -> (i32 , f64) { let x64 = x as f64 ; let mut tx : [f64 ; 1] = [0.] ; let mut ty : [f64 ; 1] = [0.] ; let ix = x . to_bits () & 0x7fffffff ; if ix < 0x4dc90fdb { let tmp = x64 * INV_PIO2 + TOINT ; # [cfg (all (target_arch = "x86" , not (target_feature = "sse2")))] let tmp = force_eval ! (tmp) ; let f_n = tmp - TOINT ; return (f_n as i32 , x64 - f_n * PIO2_1 - f_n * PIO2_1T) ; } if ix >= 0x7f800000 { return (0 , x64 - x64) ; } let sign = (x . to_bits () >> 31) != 0 ; let e0 = ((ix >> 23) - (0x7f + 23)) as i32 ; tx [0] = f32 :: from_bits (ix - (e0 << 23) as u32) as f64 ; let n = rem_pio2_large (& tx , & mut ty , e0 , 0) ; if sign { return (- n , - ty [0]) ; } (n , ty [0]) }
};
}
