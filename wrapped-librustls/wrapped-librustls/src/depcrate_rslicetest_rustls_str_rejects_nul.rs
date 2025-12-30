// Generated macro for test_rustls_str_rejects_nul (function)
macro_rules! Depcrate_rslicetest_rustls_str_rejects_nul {
() => {
// Module: crate::rslice
// Provides: {"test_rustls_str_rejects_nul"}
// Dependencies: {}
# [test] fn test_rustls_str_rejects_nul () { assert ! (matches ! (rustls_str :: try_from ("\0") , Err (NulByte { }))) ; assert ! (matches ! (rustls_str :: try_from ("abc\0") , Err (NulByte { }))) ; assert ! (matches ! (rustls_str :: try_from ("ab\0cd") , Err (NulByte { }))) ; }
};
}
