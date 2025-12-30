// Generated macro for check_fixes (function)
macro_rules! Depcrate_testscheck_fixes {
() => {
// Module: crate::tests
// Provides: {"check_fixes"}
// Dependencies: {}
# [doc = " Takes a multi-file input fixture with annotated cursor positions,"] # [doc = " and checks that:"] # [doc = "  * a diagnostic is produced"] # [doc = "  * every diagnostic fixes trigger range touches the input cursor position"] # [doc = "  * that the contents of the file containing the cursor match `after` after each diagnostic fix is applied"] pub (crate) fn check_fixes (# [rust_analyzer :: rust_fixture] ra_fixture_before : & str , ra_fixtures_after : Vec < & str > ,) { for (i , ra_fixture_after) in ra_fixtures_after . iter () . enumerate () { check_nth_fix (i , ra_fixture_before , ra_fixture_after) } }
};
}
