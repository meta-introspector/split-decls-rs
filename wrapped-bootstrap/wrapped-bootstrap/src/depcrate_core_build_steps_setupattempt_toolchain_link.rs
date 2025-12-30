// Generated macro for attempt_toolchain_link (function)
macro_rules! Depcrate_core_build_steps_setupattempt_toolchain_link {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"attempt_toolchain_link"}
// Dependencies: {}
fn attempt_toolchain_link (builder : & Builder < '_ > , stage_path : & str) { if toolchain_is_linked (builder) { return ; } if ! ensure_stage1_toolchain_placeholder_exists (stage_path) { eprintln ! ("Failed to create a template for stage 1 toolchain or confirm that it already exists") ; return ; } if try_link_toolchain (builder , stage_path) { println ! ("Added `stage1` rustup toolchain; try `cargo +stage1 build` on a separate rust project to run a newly-built toolchain") ; } else { eprintln ! ("`rustup` failed to link stage 1 build to `stage1` toolchain") ; eprintln ! ("To manually link stage 1 build to `stage1` toolchain, run:\n
            `rustup toolchain link stage1 {}`" , & stage_path) ; } }
};
}
