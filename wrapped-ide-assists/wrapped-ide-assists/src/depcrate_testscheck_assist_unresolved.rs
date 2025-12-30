// Generated macro for check_assist_unresolved (function)
macro_rules! Depcrate_testscheck_assist_unresolved {
() => {
// Module: crate::tests
// Provides: {"check_assist_unresolved"}
// Dependencies: {}
# [doc = " Check assist in unresolved state. Useful to check assists for lazy computation."] # [track_caller] pub (crate) fn check_assist_unresolved (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture : & str ,) { check (assist , ra_fixture , ExpectedResult :: Unresolved , None) ; }
};
}
