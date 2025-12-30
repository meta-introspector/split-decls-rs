// Generated macro for impl_719 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_719 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_719"}
// Dependencies: {}
impl Step for RustcPerf { # [doc = " Path to the built `collector` binary."] type Output = ToolBuildResult ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/rustc-perf") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (RustcPerf { compiler : run . builder . compiler (0 , run . builder . config . host_target) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) -> ToolBuildResult { builder . require_submodule ("src/tools/rustc-perf" , None) ; let tool = ToolBuild { build_compiler : self . compiler , target : self . target , tool : "collector" , mode : Mode :: ToolBootstrap , path : "src/tools/rustc-perf" , source_type : SourceType :: Submodule , extra_features : Vec :: new () , allow_features : "" , cargo_args : vec ! ["-p" . to_string () , "collector" . to_string ()] , artifact_kind : ToolArtifactKind :: Binary , } ; let res = builder . ensure (tool . clone ()) ; copy_link_tool_bin (builder , tool . build_compiler , tool . target , tool . mode , "rustc-fake") ; res } }
};
}
