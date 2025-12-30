// Generated macro for test_from_utf8_unchecked_short (function)
macro_rules! Depcrate_teststest_from_utf8_unchecked_short {
() => {
// Module: crate::tests
// Provides: {"test_from_utf8_unchecked_short"}
// Dependencies: {}
# [test] fn test_from_utf8_unchecked_short () { let bytes = [255 ; 10] ; let compact = unsafe { CompactString :: from_utf8_unchecked (bytes) } ; assert_eq ! (compact . len () , 10) ; assert_eq ! (compact . as_bytes () , bytes) ; }
};
}
