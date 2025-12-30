// Generated macro for macro_759 (macro)
macro_rules! Depcrate_core_build_steps_toolmacro_759 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"macro_759"}
// Dependencies: {}
tool_rustc_extended ! (Miri { path : "src/tools/miri" , tool_name : "miri" , stable : false , add_bins_to_sysroot : ["miri"] , cargo_args : & ["--all-targets"] , }) ;
};
}
