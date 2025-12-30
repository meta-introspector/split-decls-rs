// Generated macro for find_any (function)
macro_rules! Depcrate_iter_testfind_any {
() => {
// Module: crate::iter::test
// Provides: {"find_any"}
// Dependencies: {}
# [test] fn find_any () { let a : Vec < i32 > = (0 .. 1024) . collect () ; assert ! (a . par_iter () . find_any (|&& x | x % 42 == 41) . is_some ()) ; assert_eq ! (a . par_iter () . find_any (|&& x | x % 19 == 1 && x % 53 == 0) , Some (& 742_i32)) ; assert_eq ! (a . par_iter () . find_any (|&& x | x < 0) , None) ; assert ! (a . par_iter () . position_any (|& x | x % 42 == 41) . is_some ()) ; assert_eq ! (a . par_iter () . position_any (|& x | x % 19 == 1 && x % 53 == 0) , Some (742_usize)) ; assert_eq ! (a . par_iter () . position_any (|& x | x < 0) , None) ; assert ! (a . par_iter () . any (|& x | x > 1000)) ; assert ! (! a . par_iter () . any (|& x | x < 0)) ; assert ! (! a . par_iter () . all (|& x | x > 1000)) ; assert ! (a . par_iter () . all (|& x | x >= 0)) ; }
};
}
