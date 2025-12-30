// Generated macro for check_changed_files (function)
macro_rules! Depcrate_core_build_steps_toolstatecheck_changed_files {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"check_changed_files"}
// Dependencies: {}
fn check_changed_files (builder : & Builder < '_ > , toolstates : & HashMap < Box < str > , ToolState >) { let output = helpers :: git (None) . arg ("diff") . arg ("--name-status") . arg ("HEAD") . arg ("HEAD^") . run_capture (builder) . stdout () ; for (tool , submodule) in STABLE_TOOLS . iter () . chain (NIGHTLY_TOOLS . iter ()) { let changed = output . lines () . any (| l | l . starts_with ('M') && l . ends_with (submodule)) ; eprintln ! ("Verifying status of {tool}...") ; if ! changed { continue ; } eprintln ! ("This PR updated '{submodule}', verifying if status is 'test-pass'...") ; if toolstates [* tool] != ToolState :: TestPass { print_error (tool , submodule) ; } } }
};
}
