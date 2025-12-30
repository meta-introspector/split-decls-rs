// Generated macro for test_into_box_str (function)
macro_rules! Depcrate_teststest_into_box_str {
() => {
// Module: crate::tests
// Provides: {"test_into_box_str"}
// Dependencies: {}
# [test] fn test_into_box_str () { let short = "short" ; let long = "i am a long string that will be allocated on the heap" ; let s = Box :: < str > :: from (CompactString :: new (short)) ; assert_eq ! (short , &* s) ; let l = Box :: < str > :: from (CompactString :: new (long)) ; assert_eq ! (long , &* l) ; }
};
}
