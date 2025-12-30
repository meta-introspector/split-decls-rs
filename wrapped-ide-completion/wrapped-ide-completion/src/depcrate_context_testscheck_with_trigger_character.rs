// Generated macro for check_with_trigger_character (function)
macro_rules! Depcrate_context_testscheck_with_trigger_character {
() => {
// Module: crate::context::tests
// Provides: {"check_with_trigger_character"}
// Dependencies: {}
pub (crate) fn check_with_trigger_character (# [rust_analyzer :: rust_fixture] ra_fixture : & str , trigger_character : Option < char > , expect : Expect ,) { let actual = completion_list_with_trigger_character (ra_fixture , trigger_character) ; expect . assert_eq (& actual) }
};
}
