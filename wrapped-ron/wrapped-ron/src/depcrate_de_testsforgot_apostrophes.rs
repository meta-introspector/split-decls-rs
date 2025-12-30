// Generated macro for forgot_apostrophes (function)
macro_rules! Depcrate_de_testsforgot_apostrophes {
() => {
// Module: crate::de::tests
// Provides: {"forgot_apostrophes"}
// Dependencies: {}
# [test] fn forgot_apostrophes () { let bogus_struct = "(4, \"Hello)" ; let expected_err = Err (SpannedError { code : Error :: ExpectedStringEnd , span : Span { start : Position { line : 1 , col : 5 } , end : Position { line : 1 , col : 6 } , } , }) ; check_from_str_bytes_reader :: < (i32 , String) > (bogus_struct , expected_err . clone ()) ; # [cfg (feature = "internal-span-substring-test")] check_error_span_exclusive :: < (i32 , String) > (bogus_struct , expected_err , "\"") ; }
};
}
