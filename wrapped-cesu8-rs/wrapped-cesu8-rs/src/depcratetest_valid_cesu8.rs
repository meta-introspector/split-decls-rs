// Generated macro for test_valid_cesu8 (function)
macro_rules! Depcratetest_valid_cesu8 {
() => {
// Module: crate
// Provides: {"test_valid_cesu8"}
// Dependencies: {}
# [test] fn test_valid_cesu8 () { assert ! (is_valid_cesu8 ("aé日")) ; assert ! (is_valid_java_cesu8 ("aé日")) ; assert ! (! is_valid_cesu8 ("\u{10401}")) ; assert ! (! is_valid_java_cesu8 ("\u{10401}")) ; assert ! (is_valid_cesu8 ("\0\0")) ; assert ! (! is_valid_java_cesu8 ("\0\0")) ; }
};
}
