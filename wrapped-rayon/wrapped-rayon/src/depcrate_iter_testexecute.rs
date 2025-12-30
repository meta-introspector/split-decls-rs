// Generated macro for execute (function)
macro_rules! Depcrate_iter_testexecute {
() => {
// Module: crate::iter::test
// Provides: {"execute"}
// Dependencies: {}
# [test] fn execute () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let mut b = vec ! [] ; a . par_iter () . map (| & i | i + 1) . collect_into_vec (& mut b) ; let c : Vec < i32 > = (0 .. 1024) . map (| i | i + 1) . collect () ; assert_eq ! (b , c) ; }
};
}
