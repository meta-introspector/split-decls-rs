// Generated macro for test_u64_opt_len (function)
macro_rules! Depcrate_range_inclusivetest_u64_opt_len {
() => {
// Module: crate::range_inclusive
// Provides: {"test_u64_opt_len"}
// Dependencies: {}
# [test] fn test_u64_opt_len () { assert_eq ! (Some (101) , (0 ..= 100u64) . into_par_iter () . opt_len ()) ; assert_eq ! (Some (usize :: MAX) , (0 ..= usize :: MAX as u64 - 1) . into_par_iter () . opt_len ()) ; assert_eq ! (None , (0 ..= usize :: MAX as u64) . into_par_iter () . opt_len ()) ; assert_eq ! (None , (0 ..= u64 :: MAX) . into_par_iter () . opt_len ()) ; }
};
}
