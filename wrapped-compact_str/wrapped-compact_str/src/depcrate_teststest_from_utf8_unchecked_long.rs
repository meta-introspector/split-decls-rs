// Generated macro for test_from_utf8_unchecked_long (function)
macro_rules! Depcrate_teststest_from_utf8_unchecked_long {
() => {
// Module: crate::tests
// Provides: {"test_from_utf8_unchecked_long"}
// Dependencies: {}
# [test] fn test_from_utf8_unchecked_long () { let bytes = [255 ; 2048] ; let compact = unsafe { CompactString :: from_utf8_unchecked (bytes) } ; assert_eq ! (compact . len () , 2048) ; assert_eq ! (compact . as_bytes () , bytes) ; }
};
}
