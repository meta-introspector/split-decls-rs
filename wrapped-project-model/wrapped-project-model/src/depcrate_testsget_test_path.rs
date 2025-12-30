// Generated macro for get_test_path (function)
macro_rules! Depcrate_testsget_test_path {
() => {
// Module: crate::tests
// Provides: {"get_test_path"}
// Dependencies: {}
fn get_test_path (file : & str) -> Utf8PathBuf { let base = Utf8PathBuf :: from (env ! ("CARGO_MANIFEST_DIR")) ; base . join ("test_data") . join (file) }
};
}
