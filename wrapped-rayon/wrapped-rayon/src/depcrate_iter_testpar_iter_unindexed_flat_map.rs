// Generated macro for par_iter_unindexed_flat_map (function)
macro_rules! Depcrate_iter_testpar_iter_unindexed_flat_map {
() => {
// Module: crate::iter::test
// Provides: {"par_iter_unindexed_flat_map"}
// Dependencies: {}
# [test] fn par_iter_unindexed_flat_map () { let b : Vec < i64 > = (0_i64 .. 1024) . into_par_iter () . flat_map (Some) . collect () ; let c : Vec < i64 > = (0_i64 .. 1024) . flat_map (Some) . collect () ; assert_eq ! (b , c) ; }
};
}
