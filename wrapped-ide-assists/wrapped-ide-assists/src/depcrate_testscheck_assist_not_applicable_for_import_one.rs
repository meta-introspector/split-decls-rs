// Generated macro for check_assist_not_applicable_for_import_one (function)
macro_rules! Depcrate_testscheck_assist_not_applicable_for_import_one {
() => {
// Module: crate::tests
// Provides: {"check_assist_not_applicable_for_import_one"}
// Dependencies: {}
# [track_caller] pub (crate) fn check_assist_not_applicable_for_import_one (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture : & str ,) { check_with_config (TEST_CONFIG_IMPORT_ONE , assist , ra_fixture , ExpectedResult :: NotApplicable , None ,) ; }
};
}
