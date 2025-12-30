// Generated macro for execute_cloned (function)
macro_rules! Depcrate_iter_testexecute_cloned {
() => {
// Module: crate::iter::test
// Provides: {"execute_cloned"}
// Dependencies: {}
# [test] fn execute_cloned () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let mut b : Vec < i32 > = vec ! [] ; a . par_iter () . cloned () . collect_into_vec (& mut b) ; let c : Vec < i32 > = (0 .. 1024) . collect () ; assert_eq ! (b , c) ; }
};
}
