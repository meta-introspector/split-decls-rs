// Generated macro for impl_747 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_747 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_747"}
// Dependencies: {}
impl Step for LlvmBitcodeLinker { type Output = ToolBuildResult ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let builder = run . builder ; run . path ("src/tools/llvm-bitcode-linker") . default_condition (builder . tool_enabled ("llvm-bitcode-linker")) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (LlvmBitcodeLinker { build_compiler : Self :: get_build_compiler_for_target (run . builder , run . target) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> ToolBuildResult { builder . ensure (ToolBuild { build_compiler : self . build_compiler , target : self . target , tool : "llvm-bitcode-linker" , mode : Mode :: ToolTarget , path : "src/tools/llvm-bitcode-linker" , source_type : SourceType :: InTree , extra_features : vec ! [] , allow_features : "" , cargo_args : Vec :: new () , artifact_kind : ToolArtifactKind :: Binary , }) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: build ("LlvmBitcodeLinker" , self . target) . built_by (self . build_compiler)) } }
};
}
