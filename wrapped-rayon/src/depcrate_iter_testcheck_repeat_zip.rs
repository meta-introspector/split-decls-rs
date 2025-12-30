// Generated macro for check_repeat_zip (function)
macro_rules! Depcrate_iter_testcheck_repeat_zip {
() => {
// Module: crate::iter::test
// Provides: {"check_repeat_zip"}
// Dependencies: {}
# [test] fn check_repeat_zip () { let v = vec ! [4 , 4 , 4 , 4] ; let mut fours : Vec < _ > = repeat (4) . zip (v) . collect () ; assert_eq ! (fours . len () , 4) ; while let Some (item) = fours . pop () { assert_eq ! (item , (4 , 4)) ; } }
};
}
