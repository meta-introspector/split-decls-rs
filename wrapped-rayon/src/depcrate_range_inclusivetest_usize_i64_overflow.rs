// Generated macro for test_usize_i64_overflow (function)
macro_rules! Depcrate_range_inclusivetest_usize_i64_overflow {
() => {
// Module: crate::range_inclusive
// Provides: {"test_usize_i64_overflow"}
// Dependencies: {}
# [test] # [cfg (target_pointer_width = "64")] fn test_usize_i64_overflow () { use crate :: ThreadPoolBuilder ; let iter = (- 2 ..= i64 :: MAX) . into_par_iter () ; assert_eq ! (iter . opt_len () , Some (i64 :: MAX as usize + 3)) ; let pool = ThreadPoolBuilder :: new () . num_threads (8) . build () . unwrap () ; pool . install (| | assert_eq ! (iter . find_last (| _ | true) , Some (i64 :: MAX))) ; }
};
}
