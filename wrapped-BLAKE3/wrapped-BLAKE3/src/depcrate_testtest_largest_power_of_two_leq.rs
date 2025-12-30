// Generated macro for test_largest_power_of_two_leq (function)
macro_rules! Depcrate_testtest_largest_power_of_two_leq {
() => {
// Module: crate::test
// Provides: {"test_largest_power_of_two_leq"}
// Dependencies: {}
# [test] fn test_largest_power_of_two_leq () { let input_output = & [(0 , 1) , (1 , 1) , (2 , 2) , (3 , 2) , (4 , 4) , (5 , 4) , (6 , 4) , (7 , 4) , (8 , 8) , (usize :: MAX , (usize :: MAX >> 1) + 1) ,] ; for & (input , output) in input_output { assert_eq ! (output , crate :: largest_power_of_two_leq (input) , "wrong output for n={}" , input) ; } }
};
}
