// Generated macro for check_empty_flat_map_sum (function)
macro_rules! Depcrate_iter_testcheck_empty_flat_map_sum {
() => {
// Module: crate::iter::test
// Provides: {"check_empty_flat_map_sum"}
// Dependencies: {}
# [test] fn check_empty_flat_map_sum () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let empty = & a [.. 0] ; let b : i32 = a . par_iter () . flat_map (| _ | empty) . sum () ; assert_eq ! (b , 0) ; let c : i32 = empty . par_iter () . flat_map (| _ | a . par_iter ()) . sum () ; assert_eq ! (c , 0) ; }
};
}
