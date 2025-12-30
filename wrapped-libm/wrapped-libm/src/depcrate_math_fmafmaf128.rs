// Generated macro for fmaf128 (function)
macro_rules! Depcrate_math_fmafmaf128 {
() => {
// Module: crate::math::fma
// Provides: {"fmaf128"}
// Dependencies: {}
# [doc = " Fused multiply add (f128)"] # [doc = ""] # [doc = " Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision)."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaf128 (x : f128 , y : f128 , z : f128) -> f128 { generic :: fma_round (x , y , z , Round :: Nearest) . val }
};
}
