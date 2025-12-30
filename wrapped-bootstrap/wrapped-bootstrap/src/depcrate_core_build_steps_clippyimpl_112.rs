// Generated macro for impl_112 (impl)
macro_rules! Depcrate_core_build_steps_clippyimpl_112 {
() => {
// Module: crate::core::build_steps::clippy
// Provides: {"impl_112"}
// Dependencies: {}
impl Step for CodegenGcc { type Output = () ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("rustc_codegen_gcc") } fn make_run (run : RunConfig < '_ >) { let builder = run . builder ; let config = LintConfig :: new (builder) ; builder . ensure (CodegenGcc :: new (builder , run . target , config)) ; } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let build_compiler = self . build_compiler . build_compiler () ; let target = self . target ; let mut cargo = prepare_tool_cargo (builder , build_compiler , Mode :: Codegen , target , Kind :: Clippy , "compiler/rustc_codegen_gcc" , SourceType :: InTree , & [] ,) ; self . build_compiler . configure_cargo (& mut cargo) ; let _guard = builder . msg (Kind :: Clippy , "rustc_codegen_gcc" , Mode :: ToolRustcPrivate , build_compiler , target ,) ; let stamp = BuildStamp :: new (& builder . cargo_out (build_compiler , Mode :: Codegen , target)) . with_prefix ("rustc_codegen_gcc-check") ; run_cargo (builder , cargo , lint_args (builder , & self . config , & []) , & stamp , vec ! [] , true , false ,) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: clippy ("rustc_codegen_gcc" , self . target) . built_by (self . build_compiler . build_compiler ()) ,) } }
};
}
