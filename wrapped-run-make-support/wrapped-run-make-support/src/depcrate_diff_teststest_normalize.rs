// Generated macro for test_normalize (function)
macro_rules! Depcrate_diff_teststest_normalize {
() => {
// Module: crate::diff::tests
// Provides: {"test_normalize"}
// Dependencies: {}
# [test] fn test_normalize () { let expected = "
running 2 tests
..

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in $TIME
" ; let actual = "
running 2 tests
..

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
" ; diff () . expected_text ("EXPECTED_TEXT" , expected) . actual_text ("ACTUAL_TEXT" , actual) . normalize (r#"finished in \d+\.\d+s"# , "finished in $$TIME") . run () ; }
};
}
