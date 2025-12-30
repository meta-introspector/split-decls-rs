// Generated macro for get_test_name (function)
macro_rules! Depcrate_test_harnessget_test_name {
() => {
// Module: crate::test_harness
// Provides: {"get_test_name"}
// Dependencies: {}
fn get_test_name (i : & ast :: Item) -> Option < Symbol > { attr :: first_attr_value_str_by_name (& i . attrs , sym :: rustc_test_marker) }
};
}
