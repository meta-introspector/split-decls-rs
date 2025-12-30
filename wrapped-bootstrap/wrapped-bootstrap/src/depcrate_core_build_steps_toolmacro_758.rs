// Generated macro for macro_758 (macro)
macro_rules! Depcrate_core_build_steps_toolmacro_758 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"macro_758"}
// Dependencies: {}
tool_rustc_extended ! (Clippy { path : "src/tools/clippy" , tool_name : "clippy-driver" , stable : true , add_bins_to_sysroot : ["clippy-driver"] , add_features : | builder , target , features | { if builder . config . jemalloc (target) { features . push ("jemalloc" . to_string ()) ; } } }) ;
};
}
