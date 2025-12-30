// Generated macro for check_count (function)
macro_rules! Depcrate_iter_testcheck_count {
() => {
// Module: crate::iter::test
// Provides: {"check_count"}
// Dependencies: {}
# [test] fn check_count () { let c0 = (0_u32 .. 24 * 1024) . filter (| i | i % 2 == 0) . count () ; let c1 = (0_u32 .. 24 * 1024) . into_par_iter () . filter (| i | i % 2 == 0) . count () ; assert_eq ! (c0 , c1) ; }
};
}
