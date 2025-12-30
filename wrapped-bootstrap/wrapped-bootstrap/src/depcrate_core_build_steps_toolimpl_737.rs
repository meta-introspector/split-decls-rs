// Generated macro for impl_737 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_737 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_737"}
// Dependencies: {}
impl Step for WasmComponentLd { type Output = ToolBuildResult ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/wasm-component-ld") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (WasmComponentLd { build_compiler : get_tool_target_compiler (run . builder , ToolTargetBuildMode :: Build (run . target) ,) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> ToolBuildResult { builder . ensure (ToolBuild { build_compiler : self . build_compiler , target : self . target , tool : "wasm-component-ld" , mode : Mode :: ToolTarget , path : "src/tools/wasm-component-ld" , source_type : SourceType :: InTree , extra_features : vec ! [] , allow_features : "" , cargo_args : vec ! [] , artifact_kind : ToolArtifactKind :: Binary , }) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: build ("WasmComponentLd" , self . target) . built_by (self . build_compiler)) } }
};
}
