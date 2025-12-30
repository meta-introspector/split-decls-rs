// Generated macro for test_lineno (function)
macro_rules! Depcratetest_lineno {
() => {
// Module: crate
// Provides: {"test_lineno"}
// Dependencies: {}
# [test] fn test_lineno () { let mut sh = Shlex :: new ("\nfoo\nbar") ; while let Some (word) = sh . next () { if word == "bar" { assert_eq ! (sh . line_no , 3) ; } } }
};
}
