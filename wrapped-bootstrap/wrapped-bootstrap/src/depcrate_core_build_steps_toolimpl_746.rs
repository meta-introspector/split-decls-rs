// Generated macro for impl_746 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_746 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_746"}
// Dependencies: {}
impl LlvmBitcodeLinker { # [doc = " Returns `LlvmBitcodeLinker` that will be **compiled** by the passed compiler, for the given"] # [doc = " `target`."] pub fn from_build_compiler (build_compiler : Compiler , target : TargetSelection) -> Self { Self { build_compiler , target } } # [doc = " Returns `LlvmBitcodeLinker` that should be **used** by the passed compiler."] pub fn from_target_compiler (builder : & Builder < '_ > , target_compiler : Compiler) -> Self { Self { build_compiler : get_tool_target_compiler (builder , ToolTargetBuildMode :: Dist (target_compiler) ,) , target : target_compiler . host , } } # [doc = " Return a compiler that is able to build this tool for the given `target`."] pub fn get_build_compiler_for_target (builder : & Builder < '_ > , target : TargetSelection ,) -> Compiler { get_tool_target_compiler (builder , ToolTargetBuildMode :: Build (target)) } }
};
}
