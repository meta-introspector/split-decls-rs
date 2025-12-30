// Generated macro for arrays_err (function)
macro_rules! Depcrate_testsarrays_err {
() => {
// Module: crate::tests
// Provides: {"arrays_err"}
// Dependencies: {}
# [rstest] # [case :: no_tabs ("{=[u8; \t 3]}")] # [case :: no_linebreaks ("{=[u8; \n 3]}")] # [case :: too_large ("{=[u8; 9999999999999999999999999]}")] fn arrays_err (# [case] input : & str) { assert ! (parse (input , ParserMode :: Strict) . is_err ()) ; }
};
}
