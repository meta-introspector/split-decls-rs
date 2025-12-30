// Generated macro for ToolTargetBuildMode (enum)
macro_rules! Depcrate_core_build_steps_toolToolTargetBuildMode {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"ToolTargetBuildMode"}
// Dependencies: {}
# [doc = " Determines how to build a `ToolTarget`, i.e. which compiler should be used to compile it."] # [doc = " The compiler stage is automatically bumped if we need to cross-compile a stage 1 tool."] pub enum ToolTargetBuildMode { # [doc = " Build the tool for the given `target` using rustc that corresponds to the top CLI"] # [doc = " stage."] Build (TargetSelection) , # [doc = " Build the tool so that it can be attached to the sysroot of the passed compiler."] # [doc = " Since we always dist stage 2+, the compiler that builds the tool in this case has to be"] # [doc = " stage 1+."] Dist (Compiler) , }
};
}
