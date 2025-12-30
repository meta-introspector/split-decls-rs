// Generated macro for check_interleave_eq (function)
macro_rules! Depcrate_iter_testcheck_interleave_eq {
() => {
// Module: crate::iter::test
// Provides: {"check_interleave_eq"}
// Dependencies: {}
# [test] fn check_interleave_eq () { let xs : Vec < usize > = (0 .. 10) . collect () ; let ys : Vec < usize > = (10 .. 20) . collect () ; let mut actual = vec ! [] ; xs . par_iter () . interleave (& ys) . map (| & i | i) . collect_into_vec (& mut actual) ; let expected : Vec < usize > = (0 .. 10) . zip (10 .. 20) . flat_map (| (i , j) | vec ! [i , j] . into_iter ()) . collect () ; assert_eq ! (expected , actual) ; }
};
}
