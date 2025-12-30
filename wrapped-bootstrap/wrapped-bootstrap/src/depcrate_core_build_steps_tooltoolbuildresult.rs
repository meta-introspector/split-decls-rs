// Generated macro for ToolBuildResult (struct)
macro_rules! Depcrate_core_build_steps_toolToolBuildResult {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"ToolBuildResult"}
// Dependencies: {}
# [doc = " Result of the tool build process. Each `Step` in this module is responsible"] # [doc = " for using this type as `type Output = ToolBuildResult;`"] # [derive (Clone)] pub struct ToolBuildResult { # [doc = " Artifact path of the corresponding tool that was built."] pub tool_path : PathBuf , # [doc = " Compiler used to build the tool."] pub build_compiler : Compiler , }
};
}
