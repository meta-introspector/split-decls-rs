// Generated macro for macro_1 (macro)
macro_rules! Depcratemacro_1 {
() => {
// Module: crate
// Provides: {"macro_1"}
// Dependencies: {}
# [cfg (all (target_family = "wasm" , not (target_os = "wasi")))] compile_error ! ("This wasm target is unsupported by mio. If using Tokio, disable the net feature.") ;
};
}
