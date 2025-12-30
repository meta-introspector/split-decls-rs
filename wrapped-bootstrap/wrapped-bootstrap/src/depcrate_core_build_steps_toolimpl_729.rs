// Generated macro for impl_729 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_729 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_729"}
// Dependencies: {}
impl Step for Cargo { type Output = ToolBuildResult ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let builder = run . builder ; run . path ("src/tools/cargo") . default_condition (builder . tool_enabled ("cargo")) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Cargo { build_compiler : get_tool_target_compiler (run . builder , ToolTargetBuildMode :: Build (run . target) ,) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> ToolBuildResult { builder . build . require_submodule ("src/tools/cargo" , None) ; builder . std (self . build_compiler , builder . host_target) ; builder . std (self . build_compiler , self . target) ; builder . ensure (ToolBuild { build_compiler : self . build_compiler , target : self . target , tool : "cargo" , mode : Mode :: ToolTarget , path : "src/tools/cargo" , source_type : SourceType :: Submodule , extra_features : Vec :: new () , allow_features : "min_specialization,specialization" , cargo_args : Vec :: new () , artifact_kind : ToolArtifactKind :: Binary , }) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: build ("cargo" , self . target) . built_by (self . build_compiler)) } }
};
}
