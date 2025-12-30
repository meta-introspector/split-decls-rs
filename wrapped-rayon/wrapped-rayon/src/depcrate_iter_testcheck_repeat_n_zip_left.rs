// Generated macro for check_repeat_n_zip_left (function)
macro_rules! Depcrate_iter_testcheck_repeat_n_zip_left {
() => {
// Module: crate::iter::test
// Provides: {"check_repeat_n_zip_left"}
// Dependencies: {}
# [test] fn check_repeat_n_zip_left () { let v = vec ! [4 , 4 , 4 , 4] ; let mut fours : Vec < _ > = repeat_n (4 , usize :: MAX) . zip (v) . collect () ; assert_eq ! (fours . len () , 4) ; while let Some (item) = fours . pop () { assert_eq ! (item , (4 , 4)) ; } }
};
}
