// Generated macro for test_slice (function)
macro_rules! Depcrate_ser_teststest_slice {
() => {
// Module: crate::ser::tests
// Provides: {"test_slice"}
// Dependencies: {}
# [test] fn test_slice () { check_to_string_writer (& [0 , 1 , 2 , 3 , 4 , 5] [..] , "[0,1,2,3,4,5]" , "[\n    0,\n    1,\n    2,\n    3,\n    4,\n    5,\n]" ,) ; check_to_string_writer (& [0 , 1 , 2 , 3 , 4 , 5] [1 .. 4] , "[1,2,3]" , "[\n    1,\n    2,\n    3,\n]" ,) ; }
};
}
