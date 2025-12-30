// Generated macro for test_split_off (function)
macro_rules! Depcrate_teststest_split_off {
() => {
// Module: crate::tests
// Provides: {"test_split_off"}
// Dependencies: {}
# [test] fn test_split_off () { let mut vec : SmallVec < u32 , 4 > = smallvec ! [1 , 2 , 3 , 4 , 5 , 6] ; let orig_ptr = vec . as_ptr () ; let orig_capacity = vec . capacity () ; let split_off = vec . split_off (4) ; assert_eq ! (& vec [..] , & [1 , 2 , 3 , 4]) ; assert_eq ! (& split_off [..] , & [5 , 6]) ; assert_eq ! (vec . capacity () , orig_capacity) ; assert_eq ! (vec . as_ptr () , orig_ptr) ; }
};
}
