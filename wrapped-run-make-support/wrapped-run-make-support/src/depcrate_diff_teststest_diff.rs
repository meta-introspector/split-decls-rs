// Generated macro for test_diff (function)
macro_rules! Depcrate_diff_teststest_diff {
() => {
// Module: crate::diff::tests
// Provides: {"test_diff"}
// Dependencies: {}
# [test] fn test_diff () { let expected = "foo\nbar\nbaz\n" ; let actual = "foo\nbar\nbaz\n" ; diff () . expected_text ("EXPECTED_TEXT" , expected) . actual_text ("ACTUAL_TEXT" , actual) . run () ; }
};
}
