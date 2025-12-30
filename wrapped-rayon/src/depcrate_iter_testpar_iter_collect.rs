// Generated macro for par_iter_collect (function)
macro_rules! Depcrate_iter_testpar_iter_collect {
() => {
// Module: crate::iter::test
// Provides: {"par_iter_collect"}
// Dependencies: {}
# [test] fn par_iter_collect () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let b : Vec < i32 > = a . par_iter () . map (| & i | i + 1) . collect () ; let c : Vec < i32 > = (0 .. 1024) . map (| i | i + 1) . collect () ; assert_eq ! (b , c) ; }
};
}
