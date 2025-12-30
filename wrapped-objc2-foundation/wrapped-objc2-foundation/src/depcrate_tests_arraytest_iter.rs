// Generated macro for test_iter (function)
macro_rules! Depcrate_tests_arraytest_iter {
() => {
// Module: crate::tests::array
// Provides: {"test_iter"}
// Dependencies: {}
# [test] fn test_iter () { let array = sample_number_array (4) ; let vec1 = array . to_vec () ; let vec2 : Vec < _ > = array . iter () . collect () ; assert_eq ! (vec1 , vec2) ; let mut iterations = 0 ; for _ in & array { iterations += 1 ; } for _ in array . iter () { iterations += 1 ; } for _ in unsafe { array . iter_unchecked () } { iterations += 1 ; } for _ in array { iterations += 1 ; } assert_eq ! (iterations , 4 * 4) ; }
};
}
