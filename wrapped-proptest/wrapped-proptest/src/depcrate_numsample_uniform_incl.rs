// Generated macro for sample_uniform_incl (function)
macro_rules! Depcrate_numsample_uniform_incl {
() => {
// Module: crate::num
// Provides: {"sample_uniform_incl"}
// Dependencies: {}
# [doc = " Generate a random value of `X`, sampled uniformly from the closed"] # [doc = " range `[low, high]` (inclusive). Panics if `low > high`."] pub fn sample_uniform_incl < X : SampleUniform > (run : & mut TestRunner , start : X , end : X ,) -> X { Uniform :: new_inclusive (start , end) . expect ("not uniform") . sample (run . rng ()) }
};
}
