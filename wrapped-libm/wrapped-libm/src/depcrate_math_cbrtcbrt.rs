// Generated macro for cbrt (function)
macro_rules! Depcrate_math_cbrtcbrt {
() => {
// Module: crate::math::cbrt
// Provides: {"cbrt"}
// Dependencies: {}
# [doc = " Compute the cube root of the argument."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn cbrt (x : f64) -> f64 { cbrt_round (x , Round :: Nearest) . val }
};
}
