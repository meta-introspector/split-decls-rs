// Generated macro for test_into_arc (function)
macro_rules! Depcrate_teststest_into_arc {
() => {
// Module: crate::tests
// Provides: {"test_into_arc"}
// Dependencies: {}
# [test] fn test_into_arc () { let short = "short" ; let long = "i am a long string that will be allocated on the heap" ; let arc = alloc :: sync :: Arc :: < str > :: from (CompactString :: new (short)) ; assert_eq ! (short , &* arc) ; let arc = alloc :: sync :: Arc :: < str > :: from (CompactString :: new (long)) ; assert_eq ! (long , &* arc) ; }
};
}
