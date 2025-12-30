// Generated macro for check_slice_split_inclusive (function)
macro_rules! Depcrate_iter_testcheck_slice_split_inclusive {
() => {
// Module: crate::iter::test
// Provides: {"check_slice_split_inclusive"}
// Dependencies: {}
# [test] fn check_slice_split_inclusive () { let v : Vec < _ > = (0 .. 1000) . collect () ; for m in 1 .. 100 { let a : Vec < _ > = v . split_inclusive (| x | x % m == 0) . collect () ; let b : Vec < _ > = v . par_split_inclusive (| x | x % m == 0) . collect () ; assert_eq ! (a , b) ; } let slice = [10 , 40 , 33 , 20] ; let v : Vec < _ > = slice . par_split_inclusive (| num | num % 3 == 0) . collect () ; assert_eq ! (v , & [& slice [.. 3] , & slice [3 ..]]) ; let slice = [3 , 10 , 40 , 33] ; let v : Vec < _ > = slice . par_split_inclusive (| num | num % 3 == 0) . collect () ; assert_eq ! (v , & [& slice [.. 1] , & slice [1 ..]]) ; }
};
}
