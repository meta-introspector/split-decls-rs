// Generated macro for try_link_toolchain (function)
macro_rules! Depcrate_core_build_steps_setuptry_link_toolchain {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"try_link_toolchain"}
// Dependencies: {}
fn try_link_toolchain (builder : & Builder < '_ > , stage_path : & str) -> bool { command ("rustup") . args (["toolchain" , "link" , "stage1" , stage_path]) . run_capture_stdout (builder) . is_success () }
};
}
