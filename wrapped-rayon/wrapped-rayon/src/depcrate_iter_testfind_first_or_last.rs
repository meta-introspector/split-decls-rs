// Generated macro for find_first_or_last (function)
macro_rules! Depcrate_iter_testfind_first_or_last {
() => {
// Module: crate::iter::test
// Provides: {"find_first_or_last"}
// Dependencies: {}
# [test] fn find_first_or_last () { let a : Vec < i32 > = (0 .. 1024) . collect () ; assert_eq ! (a . par_iter () . find_first (|&& x | x % 42 == 41) , Some (& 41_i32)) ; assert_eq ! (a . par_iter () . find_first (|&& x | x % 19 == 1 && x % 53 == 0) , Some (& 742_i32)) ; assert_eq ! (a . par_iter () . find_first (|&& x | x < 0) , None) ; assert_eq ! (a . par_iter () . position_first (|& x | x % 42 == 41) , Some (41_usize)) ; assert_eq ! (a . par_iter () . position_first (|& x | x % 19 == 1 && x % 53 == 0) , Some (742_usize)) ; assert_eq ! (a . par_iter () . position_first (|& x | x < 0) , None) ; assert_eq ! (a . par_iter () . find_last (|&& x | x % 42 == 41) , Some (& 1007_i32)) ; assert_eq ! (a . par_iter () . find_last (|&& x | x % 19 == 1 && x % 53 == 0) , Some (& 742_i32)) ; assert_eq ! (a . par_iter () . find_last (|&& x | x < 0) , None) ; assert_eq ! (a . par_iter () . position_last (|& x | x % 42 == 41) , Some (1007_usize)) ; assert_eq ! (a . par_iter () . position_last (|& x | x % 19 == 1 && x % 53 == 0) , Some (742_usize)) ; assert_eq ! (a . par_iter () . position_last (|& x | x < 0) , None) ; }
};
}
