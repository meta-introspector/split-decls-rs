// Generated macro for test_unterminated (function)
macro_rules! Depcrate_teststest_unterminated {
() => {
// Module: crate::tests
// Provides: {"test_unterminated"}
// Dependencies: {}
# [test] fn test_unterminated () { check_raw_str (r#"#"abc"# , Err (RawStrError :: NoTerminator { expected : 1 , found : 0 , possible_terminator_offset : None }) ,) ; check_raw_str (r###"##"abc"#"### , Err (RawStrError :: NoTerminator { expected : 2 , found : 1 , possible_terminator_offset : Some (7) , }) ,) ; check_raw_str (r###"##"abc#"### , Err (RawStrError :: NoTerminator { expected : 2 , found : 0 , possible_terminator_offset : None }) ,) }
};
}
