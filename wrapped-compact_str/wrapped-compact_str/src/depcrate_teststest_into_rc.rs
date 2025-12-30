// Generated macro for test_into_rc (function)
macro_rules! Depcrate_teststest_into_rc {
() => {
// Module: crate::tests
// Provides: {"test_into_rc"}
// Dependencies: {}
# [test] fn test_into_rc () { let short = "short" ; let long = "i am a long string that will be allocated on the heap" ; let rc = alloc :: rc :: Rc :: < str > :: from (CompactString :: new (short)) ; assert_eq ! (short , &* rc) ; let rc = alloc :: rc :: Rc :: < str > :: from (CompactString :: new (long)) ; assert_eq ! (long , &* rc) ; }
};
}
