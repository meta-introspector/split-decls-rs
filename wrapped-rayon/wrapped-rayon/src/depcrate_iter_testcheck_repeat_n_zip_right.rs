// Generated macro for check_repeat_n_zip_right (function)
macro_rules! Depcrate_iter_testcheck_repeat_n_zip_right {
() => {
// Module: crate::iter::test
// Provides: {"check_repeat_n_zip_right"}
// Dependencies: {}
# [test] fn check_repeat_n_zip_right () { let v = vec ! [4 , 4 , 4 , 4] ; let mut fours : Vec < _ > = v . into_par_iter () . zip (repeat_n (4 , usize :: MAX)) . collect () ; assert_eq ! (fours . len () , 4) ; while let Some (item) = fours . pop () { assert_eq ! (item , (4 , 4)) ; } }
};
}
