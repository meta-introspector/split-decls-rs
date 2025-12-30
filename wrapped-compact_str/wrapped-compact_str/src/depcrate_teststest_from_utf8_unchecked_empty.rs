// Generated macro for test_from_utf8_unchecked_empty (function)
macro_rules! Depcrate_teststest_from_utf8_unchecked_empty {
() => {
// Module: crate::tests
// Provides: {"test_from_utf8_unchecked_empty"}
// Dependencies: {}
# [test] fn test_from_utf8_unchecked_empty () { let bytes = [255 ; 0] ; let compact = unsafe { CompactString :: from_utf8_unchecked (bytes) } ; assert_eq ! (compact . len () , 0) ; assert_eq ! (compact . as_bytes () , bytes) ; }
};
}
