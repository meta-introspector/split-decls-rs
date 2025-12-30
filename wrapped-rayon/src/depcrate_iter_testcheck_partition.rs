// Generated macro for check_partition (function)
macro_rules! Depcrate_iter_testcheck_partition {
() => {
// Module: crate::iter::test
// Provides: {"check_partition"}
// Dependencies: {}
# [test] fn check_partition () { let (a , b) : (Vec < _ > , Vec < _ >) = (0 .. 1024) . into_par_iter () . partition (| & i | i % 3 == 0) ; let (c , d) : (Vec < _ > , Vec < _ >) = (0 .. 1024) . partition (| & i | i % 3 == 0) ; assert_eq ! (a , c) ; assert_eq ! (b , d) ; }
};
}
