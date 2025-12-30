// Generated macro for check_raw_str (function)
macro_rules! Depcrate_testscheck_raw_str {
() => {
// Module: crate::tests
// Provides: {"check_raw_str"}
// Dependencies: {}
fn check_raw_str (s : & str , expected : Result < u8 , RawStrError >) { let s = & format ! ("r{}" , s) ; let mut cursor = Cursor :: new (s , FrontmatterAllowed :: No) ; cursor . bump () ; let res = cursor . raw_double_quoted_string (0) ; assert_eq ! (res , expected) ; }
};
}
