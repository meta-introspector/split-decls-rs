// Generated macro for use_host_linker (function)
macro_rules! Depcrate_utils_helpersuse_host_linker {
() => {
// Module: crate::utils::helpers
// Provides: {"use_host_linker"}
// Dependencies: {}
pub fn use_host_linker (target : TargetSelection) -> bool { ! (target . contains ("emscripten") || target . contains ("wasm32") || target . contains ("nvptx") || target . contains ("fortanix") || target . contains ("fuchsia") || target . contains ("bpf") || target . contains ("switch")) }
};
}
