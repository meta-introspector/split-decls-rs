// Generated macro for check_linked_list (function)
macro_rules! Depcrate_iter_testcheck_linked_list {
() => {
// Module: crate::iter::test
// Provides: {"check_linked_list"}
// Dependencies: {}
# [test] fn check_linked_list () { use std :: collections :: LinkedList ; let mut a : LinkedList < i32 > = (0 .. 10) . collect () ; assert_eq ! (45 , a . par_iter () . sum ::< i32 > ()) ; a . par_iter_mut () . for_each (| x | * x = - * x) ; assert_eq ! (- 45 , a . into_par_iter () . sum ::< i32 > ()) ; }
};
}
