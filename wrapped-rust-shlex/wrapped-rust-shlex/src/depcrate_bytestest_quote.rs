// Generated macro for test_quote (function)
macro_rules! Depcrate_bytestest_quote {
() => {
// Module: crate::bytes
// Provides: {"test_quote"}
// Dependencies: {}
# [test] # [allow (deprecated)] fn test_quote () { assert_eq ! (quote (INVALID_UTF8) , INVALID_UTF8_SINGLEQUOTED) ; assert_eq ! (quote (b"") , & b"''" [..]) ; assert_eq ! (quote (b"foobar") , & b"foobar" [..]) ; assert_eq ! (quote (b"foo bar") , & b"'foo bar'" [..]) ; assert_eq ! (quote (b"'\"") , & b"\"'\\\"\"" [..]) ; assert_eq ! (quote (b"") , & b"''" [..]) ; }
};
}
