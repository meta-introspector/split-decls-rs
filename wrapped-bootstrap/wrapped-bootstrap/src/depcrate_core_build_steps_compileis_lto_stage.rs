// Generated macro for is_lto_stage (function)
macro_rules! Depcrate_core_build_steps_compileis_lto_stage {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"is_lto_stage"}
// Dependencies: {}
# [doc = " We only use LTO for stage 2+, to speed up build time of intermediate stages."] pub fn is_lto_stage (build_compiler : & Compiler) -> bool { build_compiler . stage != 0 }
};
}
