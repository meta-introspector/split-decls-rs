// Generated macro for par_iter_collect_linked_list_flat_map_filter (function)
macro_rules! Depcrate_iter_testpar_iter_collect_linked_list_flat_map_filter {
() => {
// Module: crate::iter::test
// Provides: {"par_iter_collect_linked_list_flat_map_filter"}
// Dependencies: {}
# [test] fn par_iter_collect_linked_list_flat_map_filter () { let b : LinkedList < i32 > = (0_i32 .. 1024) . into_par_iter () . flat_map (| i | 0 .. i) . filter (| & i | i % 2 == 0) . collect () ; let c : LinkedList < i32 > = (0_i32 .. 1024) . flat_map (| i | 0 .. i) . filter (| & i | i % 2 == 0) . collect () ; assert_eq ! (b , c) ; }
};
}
