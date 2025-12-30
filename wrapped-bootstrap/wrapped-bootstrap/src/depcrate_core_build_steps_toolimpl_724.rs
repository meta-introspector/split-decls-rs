// Generated macro for impl_724 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_724 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_724"}
// Dependencies: {}
impl Step for RemoteTestServer { type Output = ToolBuildResult ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/remote-test-server") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (RemoteTestServer { build_compiler : get_tool_target_compiler (run . builder , ToolTargetBuildMode :: Build (run . target) ,) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> ToolBuildResult { builder . ensure (ToolBuild { build_compiler : self . build_compiler , target : self . target , tool : "remote-test-server" , mode : Mode :: ToolTarget , path : "src/tools/remote-test-server" , source_type : SourceType :: InTree , extra_features : Vec :: new () , allow_features : "" , cargo_args : Vec :: new () , artifact_kind : ToolArtifactKind :: Binary , }) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: build ("remote-test-server" , self . target) . built_by (self . build_compiler)) } }
};
}
