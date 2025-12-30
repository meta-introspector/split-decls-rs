// Generated macro for par_iter_collect_result (function)
macro_rules! Depcrate_iter_testpar_iter_collect_result {
() => {
// Module: crate::iter::test
// Provides: {"par_iter_collect_result"}
// Dependencies: {}
# [test] fn par_iter_collect_result () { let a : Result < Vec < _ > , () > = (0_i32 .. 2048) . map (Ok) . collect () ; let b : Result < Vec < _ > , () > = (0_i32 .. 2048) . into_par_iter () . map (Ok) . collect () ; assert_eq ! (a , b) ; let c : Result < Vec < _ > , _ > = (0_i32 .. 2048) . into_par_iter () . map (| x | if x == 1234 { Err (x) } else { Ok (x) }) . collect () ; assert_eq ! (c , Err (1234)) ; let d : Result < Vec < _ > , _ > = (0_i32 .. 2048) . into_par_iter () . map (| x | if x % 100 == 99 { Err (x) } else { Ok (x) }) . collect () ; assert_eq ! (d . map_err (| x | x % 100) , Err (99)) ; }
};
}
