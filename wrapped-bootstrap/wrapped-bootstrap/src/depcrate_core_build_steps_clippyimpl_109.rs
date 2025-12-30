// Generated macro for impl_109 (impl)
macro_rules! Depcrate_core_build_steps_clippyimpl_109 {
() => {
// Module: crate::core::build_steps::clippy
// Provides: {"impl_109"}
// Dependencies: {}
impl Step for Rustc { type Output = () ; const IS_HOST : bool = true ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . crate_or_deps ("rustc-main") . path ("compiler") } fn make_run (run : RunConfig < '_ >) { let builder = run . builder ; let crates = run . make_run_crates (Alias :: Compiler) ; let config = LintConfig :: new (run . builder) ; run . builder . ensure (Rustc :: new (builder , run . target , config , crates)) ; } fn run (self , builder : & Builder < '_ >) { let build_compiler = self . build_compiler . build_compiler () ; let target = self . target ; let mut cargo = builder :: Cargo :: new (builder , build_compiler , Mode :: Rustc , SourceType :: InTree , target , Kind :: Clippy ,) ; rustc_cargo (builder , & mut cargo , target , & build_compiler , & self . crates) ; self . build_compiler . configure_cargo (& mut cargo) ; for krate in & * self . crates { cargo . arg ("-p") . arg (krate) ; } let _guard = builder . msg (Kind :: Clippy , format_args ! ("compiler{}" , crate_description (& self . crates)) , Mode :: Rustc , build_compiler , target ,) ; run_cargo (builder , cargo , lint_args (builder , & self . config , IGNORED_RULES_FOR_STD_AND_RUSTC) , & build_stamp :: librustc_stamp (builder , build_compiler , target) , vec ! [] , true , false ,) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: clippy ("rustc" , self . target) . built_by (self . build_compiler . build_compiler ()) ,) } }
};
}
