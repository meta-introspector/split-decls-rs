// Generated macro for get_tool_target_compiler (function)
macro_rules! Depcrate_core_build_steps_toolget_tool_target_compiler {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"get_tool_target_compiler"}
// Dependencies: {}
# [doc = " Returns compiler that is able to compile a `ToolTarget` tool with the given `mode`."] pub (crate) fn get_tool_target_compiler (builder : & Builder < '_ > , mode : ToolTargetBuildMode ,) -> Compiler { let (target , build_compiler_stage) = match mode { ToolTargetBuildMode :: Build (target) => { assert ! (builder . top_stage > 0) ; (target , builder . top_stage - 1) } ToolTargetBuildMode :: Dist (target_compiler) => { assert ! (target_compiler . stage > 0) ; (target_compiler . host , target_compiler . stage - 1) } } ; let compiler = if builder . host_target == target { builder . compiler (build_compiler_stage , builder . host_target) } else { let build_compiler = builder . compiler (build_compiler_stage . max (1) , builder . host_target) ; builder . std (build_compiler , builder . host_target) ; build_compiler } ; builder . std (compiler , target) ; compiler }
};
}
