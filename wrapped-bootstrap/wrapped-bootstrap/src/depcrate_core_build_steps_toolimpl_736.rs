// Generated macro for impl_736 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_736 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_736"}
// Dependencies: {}
impl WasmComponentLd { # [doc = " Returns `WasmComponentLd` that should be **used** by the passed compiler."] pub fn for_use_by_compiler (builder : & Builder < '_ > , target_compiler : Compiler) -> Self { Self { build_compiler : get_tool_target_compiler (builder , ToolTargetBuildMode :: Dist (target_compiler) ,) , target : target_compiler . host , } } }
};
}
