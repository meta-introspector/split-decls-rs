// Generated macro for test_join (function)
macro_rules! Depcrate_bytestest_join {
() => {
// Module: crate::bytes
// Provides: {"test_join"}
// Dependencies: {}
# [test] # [allow (deprecated)] fn test_join () { assert_eq ! (join (vec ! [INVALID_UTF8]) , INVALID_UTF8_SINGLEQUOTED) ; assert_eq ! (join (vec ! []) , & b"" [..]) ; assert_eq ! (join (vec ! [& b"" [..]]) , b"''") ; }
};
}
