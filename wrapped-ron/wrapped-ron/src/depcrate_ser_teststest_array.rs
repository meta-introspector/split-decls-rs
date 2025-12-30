// Generated macro for test_array (function)
macro_rules! Depcrate_ser_teststest_array {
() => {
// Module: crate::ser::tests
// Provides: {"test_array"}
// Dependencies: {}
# [test] fn test_array () { let empty : [i32 ; 0] = [] ; check_to_string_writer (& empty , "()" , "()") ; let empty_ref : & [i32] = & empty ; check_to_string_writer (& empty_ref , "[]" , "[]") ; check_to_string_writer (& [2 , 3 , 4i32] , "(2,3,4)" , "(2, 3, 4)") ; check_to_string_writer (& (& [2 , 3 , 4i32] as & [i32]) , "[2,3,4]" , "[\n    2,\n    3,\n    4,\n]" ,) ; }
};
}
