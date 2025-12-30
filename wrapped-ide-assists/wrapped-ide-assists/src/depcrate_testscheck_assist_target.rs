// Generated macro for check_assist_target (function)
macro_rules! Depcrate_testscheck_assist_target {
() => {
// Module: crate::tests
// Provides: {"check_assist_target"}
// Dependencies: {}
# [track_caller] pub (crate) fn check_assist_target (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture : & str , target : & str ,) { check (assist , ra_fixture , ExpectedResult :: Target (target) , None) ; }
};
}
