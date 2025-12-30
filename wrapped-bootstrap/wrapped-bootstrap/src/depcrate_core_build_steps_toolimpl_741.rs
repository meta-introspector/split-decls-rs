// Generated macro for impl_741 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_741 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_741"}
// Dependencies: {}
impl Step for RustAnalyzer { type Output = ToolBuildResult ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let builder = run . builder ; run . path ("src/tools/rust-analyzer") . default_condition (builder . tool_enabled ("rust-analyzer")) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (RustAnalyzer { compilers : RustcPrivateCompilers :: new (run . builder , run . builder . top_stage , run . target) , }) ; } fn run (self , builder : & Builder < '_ >) -> ToolBuildResult { let build_compiler = self . compilers . build_compiler ; let target = self . compilers . target () ; builder . ensure (ToolBuild { build_compiler , target , tool : "rust-analyzer" , mode : Mode :: ToolRustcPrivate , path : "src/tools/rust-analyzer" , extra_features : vec ! ["in-rust-tree" . to_owned ()] , source_type : SourceType :: InTree , allow_features : RustAnalyzer :: ALLOW_FEATURES , cargo_args : Vec :: new () , artifact_kind : ToolArtifactKind :: Binary , }) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: build ("rust-analyzer" , self . compilers . target ()) . built_by (self . compilers . build_compiler) ,) } }
};
}
