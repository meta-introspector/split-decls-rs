// Generated macro for check_fixes_unordered (function)
macro_rules! Depcrate_testscheck_fixes_unordered {
() => {
// Module: crate::tests
// Provides: {"check_fixes_unordered"}
// Dependencies: {}
pub (crate) fn check_fixes_unordered (# [rust_analyzer :: rust_fixture] ra_fixture_before : & str , ra_fixtures_after : Vec < & str > ,) { for ra_fixture_after in ra_fixtures_after . iter () { check_has_fix (ra_fixture_before , ra_fixture_after) } }
};
}
