// Generated macro for test_with_capacity (function)
macro_rules! Depcrate_tests_mutable_stringtest_with_capacity {
() => {
// Module: crate::tests::mutable_string
// Provides: {"test_with_capacity"}
// Dependencies: {}
# [test] # [allow (clippy :: deref_addrof)] fn test_with_capacity () { let s = NSMutableString :: stringWithCapacity (3) ; * & mut & * s += & NSString :: from_str ("abc") ; * & mut & * s += & NSString :: from_str ("def") ; assert_eq ! (& s . to_string () , "abcdef") ; }
};
}
