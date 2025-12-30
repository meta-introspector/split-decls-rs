// Generated macro for execute_unindexed_range (function)
macro_rules! Depcrate_iter_testexecute_unindexed_range {
() => {
// Module: crate::iter::test
// Provides: {"execute_unindexed_range"}
// Dependencies: {}
# [test] fn execute_unindexed_range () { let a = 0i64 .. 1024 ; let b : LinkedList < i64 > = a . into_par_iter () . map (| i | i + 1) . collect () ; let c : LinkedList < i64 > = (0 .. 1024) . map (| i | i + 1) . collect () ; assert_eq ! (b , c) ; }
};
}
