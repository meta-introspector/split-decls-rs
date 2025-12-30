// Generated macro for check_binary_heap (function)
macro_rules! Depcrate_iter_testcheck_binary_heap {
() => {
// Module: crate::iter::test
// Provides: {"check_binary_heap"}
// Dependencies: {}
# [test] fn check_binary_heap () { use std :: collections :: BinaryHeap ; let a : BinaryHeap < i32 > = (0 .. 10) . collect () ; assert_eq ! (45 , a . par_iter () . sum ::< i32 > ()) ; assert_eq ! (45 , a . into_par_iter () . sum ::< i32 > ()) ; }
};
}
