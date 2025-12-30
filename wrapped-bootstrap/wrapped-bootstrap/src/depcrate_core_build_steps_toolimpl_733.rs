// Generated macro for impl_733 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_733 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_733"}
// Dependencies: {}
impl Step for LldWrapper { type Output = BuiltLldWrapper ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/lld-wrapper") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (LldWrapper { build_compiler : get_tool_target_compiler (run . builder , ToolTargetBuildMode :: Build (run . target) ,) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let lld_dir = builder . ensure (llvm :: Lld { target : self . target }) ; let tool = builder . ensure (ToolBuild { build_compiler : self . build_compiler , target : self . target , tool : "lld-wrapper" , mode : Mode :: ToolTarget , path : "src/tools/lld-wrapper" , source_type : SourceType :: InTree , extra_features : Vec :: new () , allow_features : "" , cargo_args : Vec :: new () , artifact_kind : ToolArtifactKind :: Binary , }) ; BuiltLldWrapper { tool , lld_dir } } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: build ("LldWrapper" , self . target) . built_by (self . build_compiler)) } }
};
}
