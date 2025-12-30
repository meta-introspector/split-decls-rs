// Generated macro for check_extend_heap (function)
macro_rules! Depcrate_iter_testcheck_extend_heap {
() => {
// Module: crate::iter::test
// Provides: {"check_extend_heap"}
// Dependencies: {}
# [test] fn check_extend_heap () { let mut serial : BinaryHeap < _ > = Default :: default () ; let mut parallel : BinaryHeap < _ > = Default :: default () ; let v : Vec < _ > = (0 .. 128) . collect () ; serial . extend (& v) ; parallel . par_extend (& v) ; assert_eq ! (serial . clone () . into_sorted_vec () , parallel . clone () . into_sorted_vec ()) ; serial . extend (- 128 .. 0) ; parallel . par_extend (- 128 .. 0) ; assert_eq ! (serial . into_sorted_vec () , parallel . into_sorted_vec ()) ; }
};
}
