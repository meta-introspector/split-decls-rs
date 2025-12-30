// Generated macro for get_browser_ui_test_version (function)
macro_rules! Depcrate_core_build_steps_testget_browser_ui_test_version {
() => {
// Module: crate::core::build_steps::test
// Provides: {"get_browser_ui_test_version"}
// Dependencies: {}
fn get_browser_ui_test_version (builder : & Builder < '_ > , npm : & Path) -> Option < String > { get_browser_ui_test_version_inner (builder , npm , false) . or_else (| | get_browser_ui_test_version_inner (builder , npm , true)) }
};
}
