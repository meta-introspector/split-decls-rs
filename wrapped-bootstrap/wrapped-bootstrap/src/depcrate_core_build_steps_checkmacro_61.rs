// Generated macro for macro_61 (macro)
macro_rules! Depcrate_core_build_steps_checkmacro_61 {
() => {
// Module: crate::core::build_steps::check
// Provides: {"macro_61"}
// Dependencies: {}
tool_check_step ! (RustAnalyzer { path : "src/tools/rust-analyzer" , mode : | _builder | Mode :: ToolRustcPrivate , allow_features : tool :: RustAnalyzer :: ALLOW_FEATURES , enable_features : ["in-rust-tree"] , }) ;
};
}
