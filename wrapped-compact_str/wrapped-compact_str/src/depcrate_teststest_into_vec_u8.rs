// Generated macro for test_into_vec_u8 (function)
macro_rules! Depcrate_teststest_into_vec_u8 {
() => {
// Module: crate::tests
// Provides: {"test_into_vec_u8"}
// Dependencies: {}
# [test] fn test_into_vec_u8 () { let short = "short" ; let long = "i am a long string that will be allocated on the heap" ; let s = Vec :: < u8 > :: from (CompactString :: new (short)) ; assert_eq ! (& s , short . as_bytes ()) ; let l = Vec :: < u8 > :: from (CompactString :: new (long)) ; assert_eq ! (& l , long . as_bytes ()) ; }
};
}
