// Generated macro for check_interleave_shortest (function)
macro_rules! Depcrate_iter_testcheck_interleave_shortest {
() => {
// Module: crate::iter::test
// Provides: {"check_interleave_shortest"}
// Dependencies: {}
# [test] fn check_interleave_shortest () { let cases : Vec < (Vec < usize > , Vec < usize > , Vec < usize >) > = vec ! [((0 .. 9) . collect () , vec ! [10] , vec ! [0 , 10 , 1]) , (vec ! [10] , (0 .. 9) . collect () , vec ! [10 , 0]) , ((0 .. 5) . collect () , (5 .. 10) . collect () , (0 .. 5) . zip (5 .. 10) . flat_map (| (i , j) | vec ! [i , j] . into_iter ()) . collect () ,) , (vec ! [] , (0 .. 9) . collect () , vec ! []) , ((0 .. 9) . collect () , vec ! [] , vec ! [0]) , ((0 .. 50) . collect () , (50 .. 100) . collect () , (0 .. 50) . zip (50 .. 100) . flat_map (| (i , j) | vec ! [i , j] . into_iter ()) . collect () ,) ,] ; for (i , (xs , ys , expected)) in cases . into_iter () . enumerate () { let mut res = vec ! [] ; xs . par_iter () . interleave_shortest (& ys) . map (| & i | i) . collect_into_vec (& mut res) ; assert_eq ! (expected , res , "Case {i} failed") ; res . truncate (0) ; xs . par_iter () . interleave_shortest (& ys) . rev () . map (| & i | i) . collect_into_vec (& mut res) ; assert_eq ! (expected . into_iter () . rev () . collect ::< Vec < usize >> () , res , "Case {i} reversed failed") ; } }
};
}
