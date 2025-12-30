// Generated macro for check_sum_filtermap_ints (function)
macro_rules! Depcrate_iter_testcheck_sum_filtermap_ints {
() => {
// Module: crate::iter::test
// Provides: {"check_sum_filtermap_ints"}
// Dependencies: {}
# [test] fn check_sum_filtermap_ints () { let a : Vec < i32 > = vec ! [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10] ; let par_sum_evens : u32 = a . par_iter () . filter_map (| & x | if (x & 1) == 0 { Some (x as u32) } else { None }) . sum () ; let seq_sum_evens = a . iter () . filter_map (| & x | if (x & 1) == 0 { Some (x as u32) } else { None }) . sum () ; assert_eq ! (par_sum_evens , seq_sum_evens) ; }
};
}
