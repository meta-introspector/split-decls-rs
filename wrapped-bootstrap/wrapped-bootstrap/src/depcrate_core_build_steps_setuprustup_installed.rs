// Generated macro for rustup_installed (function)
macro_rules! Depcrate_core_build_steps_setuprustup_installed {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"rustup_installed"}
// Dependencies: {}
fn rustup_installed (builder : & Builder < '_ >) -> bool { let mut rustup = command ("rustup") ; rustup . arg ("--version") ; rustup . allow_failure () . run_in_dry_run () . run_capture_stdout (builder) . is_success () }
};
}
