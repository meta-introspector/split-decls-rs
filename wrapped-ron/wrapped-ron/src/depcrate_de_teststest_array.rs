// Generated macro for test_array (function)
macro_rules! Depcrate_de_teststest_array {
() => {
// Module: crate::de::tests
// Provides: {"test_array"}
// Dependencies: {}
# [test] fn test_array () { check_from_str_bytes_reader :: < [i32 ; 0] > ("()" , Ok ([])) ; check_from_str_bytes_reader ("[]" , Ok (Vec :: < i32 > :: new ())) ; check_from_str_bytes_reader ("(2,3,4,)" , Ok ([2 , 3 , 4i32])) ; check_from_str_bytes_reader ("[2,3,4,]" , Ok ([2 , 3 , 4i32] . to_vec ())) ; }
};
}
