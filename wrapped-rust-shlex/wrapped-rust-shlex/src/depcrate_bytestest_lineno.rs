// Generated macro for test_lineno (function)
macro_rules! Depcrate_bytestest_lineno {
() => {
// Module: crate::bytes
// Provides: {"test_lineno"}
// Dependencies: {}
# [test] fn test_lineno () { let mut sh = Shlex :: new (b"\nfoo\nbar") ; while let Some (word) = sh . next () { if word == b"bar" { assert_eq ! (sh . line_no , 3) ; } } }
};
}
