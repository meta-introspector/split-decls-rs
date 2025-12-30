// Generated macro for f64_to_i64 (function)
macro_rules! Depcrate_binomialf64_to_i64 {
() => {
// Module: crate::binomial
// Provides: {"f64_to_i64"}
// Dependencies: {}
# [doc = " Convert a `f64` to an `i64`, panicking on overflow."] fn f64_to_i64 (x : f64) -> i64 { assert ! (x < (i64 :: MAX as f64)) ; x as i64 }
};
}
