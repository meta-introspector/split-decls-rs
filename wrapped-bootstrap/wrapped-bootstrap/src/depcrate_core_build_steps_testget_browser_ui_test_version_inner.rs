// Generated macro for get_browser_ui_test_version_inner (function)
macro_rules! Depcrate_core_build_steps_testget_browser_ui_test_version_inner {
() => {
// Module: crate::core::build_steps::test
// Provides: {"get_browser_ui_test_version_inner"}
// Dependencies: {}
fn get_browser_ui_test_version_inner (builder : & Builder < '_ > , npm : & Path , global : bool ,) -> Option < String > { let mut command = command (npm) ; command . arg ("list") . arg ("--parseable") . arg ("--long") . arg ("--depth=0") ; if global { command . arg ("--global") ; } let lines = command . allow_failure () . run_capture (builder) . stdout () ; lines . lines () . find_map (| l | l . split (':') . nth (1) ? . strip_prefix ("browser-ui-test@")) . map (| v | v . to_owned ()) }
};
}
