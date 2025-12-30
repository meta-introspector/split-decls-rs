// Generated macro for check_slice_split_mut (function)
macro_rules! Depcrate_iter_testcheck_slice_split_mut {
() => {
// Module: crate::iter::test
// Provides: {"check_slice_split_mut"}
// Dependencies: {}
# [test] fn check_slice_split_mut () { let mut v1 : Vec < _ > = (0 .. 1000) . collect () ; let mut v2 = v1 . clone () ; for m in 1 .. 100 { let a : Vec < _ > = v1 . split_mut (| x | x % m == 0) . collect () ; let b : Vec < _ > = v2 . par_split_mut (| x | x % m == 0) . collect () ; assert_eq ! (a , b) ; } let mut v = [10 , 40 , 30 , 20 , 60 , 50] ; v . par_split_mut (| num | num % 3 == 0) . for_each (| group | { group [0] = 1 ; }) ; assert_eq ! (v , [1 , 40 , 30 , 1 , 60 , 1]) ; }
};
}
