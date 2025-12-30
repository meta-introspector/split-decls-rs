// Generated macro for par_iter_collect_linked_list (function)
macro_rules! Depcrate_iter_testpar_iter_collect_linked_list {
() => {
// Module: crate::iter::test
// Provides: {"par_iter_collect_linked_list"}
// Dependencies: {}
# [test] fn par_iter_collect_linked_list () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let b : LinkedList < _ > = a . par_iter () . map (| & i | (i , format ! ("{i}"))) . collect () ; let c : LinkedList < _ > = a . iter () . map (| & i | (i , format ! ("{i}"))) . collect () ; assert_eq ! (b , c) ; }
};
}
