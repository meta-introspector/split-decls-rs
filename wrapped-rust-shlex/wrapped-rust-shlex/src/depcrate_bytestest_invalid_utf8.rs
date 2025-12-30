// Generated macro for test_invalid_utf8 (function)
macro_rules! Depcrate_bytestest_invalid_utf8 {
() => {
// Module: crate::bytes
// Provides: {"test_invalid_utf8"}
// Dependencies: {}
# [test] # [allow (invalid_from_utf8)] fn test_invalid_utf8 () { assert ! (core :: str :: from_utf8 (INVALID_UTF8) . is_err ()) ; }
};
}
