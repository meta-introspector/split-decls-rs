// Generated macro for store_str (function)
macro_rules! Depcrate_tests_valuestore_str {
() => {
// Module: crate::tests::value
// Provides: {"store_str"}
// Dependencies: {}
# [test] fn store_str () { let s = "abc" ; let val = NSValue :: new (s . as_ptr ()) ; assert ! (val . contains_encoding ::<* const u8 > ()) ; let slice = unsafe { slice :: from_raw_parts (val . get () , s . len ()) } ; let s2 = str :: from_utf8 (slice) . unwrap () ; assert_eq ! (s2 , s) ; }
};
}
