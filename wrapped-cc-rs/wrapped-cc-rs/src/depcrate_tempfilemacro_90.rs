// Generated macro for macro_90 (macro)
macro_rules! Depcrate_tempfilemacro_90 {
() => {
// Module: crate::tempfile
// Provides: {"macro_90"}
// Dependencies: {}
# [cfg (not (any (unix , target_family = "wasm" , windows)))] compile_error ! ("Your system is not supported since cc cannot create named tempfile") ;
};
}
