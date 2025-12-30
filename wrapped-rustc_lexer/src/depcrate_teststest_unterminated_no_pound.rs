// Generated macro for test_unterminated_no_pound (function)
macro_rules! Depcrate_teststest_unterminated_no_pound {
() => {
// Module: crate::tests
// Provides: {"test_unterminated_no_pound"}
// Dependencies: {}
# [test] fn test_unterminated_no_pound () { check_raw_str (r#"""# , Err (RawStrError :: NoTerminator { expected : 0 , found : 0 , possible_terminator_offset : None }) ,) ; }
};
}
