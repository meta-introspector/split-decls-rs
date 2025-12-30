// Generated macro for impl_722 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_722 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_722"}
// Dependencies: {}
impl Step for ErrorIndex { type Output = ToolBuildResult ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/error_index_generator") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (ErrorIndex { compilers : RustcPrivateCompilers :: new (run . builder , run . builder . top_stage , run . builder . host_target ,) , }) ; } fn run (self , builder : & Builder < '_ >) -> ToolBuildResult { builder . ensure (ToolBuild { build_compiler : self . compilers . build_compiler , target : self . compilers . target () , tool : "error_index_generator" , mode : Mode :: ToolRustcPrivate , path : "src/tools/error_index_generator" , source_type : SourceType :: InTree , extra_features : Vec :: new () , allow_features : "" , cargo_args : Vec :: new () , artifact_kind : ToolArtifactKind :: Binary , }) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: build ("error-index" , self . compilers . target ()) . built_by (self . compilers . build_compiler) ,) } }
};
}
