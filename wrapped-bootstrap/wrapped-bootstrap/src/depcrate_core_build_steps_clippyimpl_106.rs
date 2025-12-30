// Generated macro for impl_106 (impl)
macro_rules! Depcrate_core_build_steps_clippyimpl_106 {
() => {
// Module: crate::core::build_steps::clippy
// Provides: {"impl_106"}
// Dependencies: {}
impl Step for Std { type Output = () ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . crate_or_deps ("sysroot") . path ("library") } fn make_run (run : RunConfig < '_ >) { let crates = std_crates_for_run_make (& run) ; let config = LintConfig :: new (run . builder) ; run . builder . ensure (Std :: new (run . builder , run . target , config , crates)) ; } fn run (self , builder : & Builder < '_ >) { let target = self . target ; let build_compiler = self . build_compiler ; let mut cargo = builder :: Cargo :: new (builder , build_compiler , Mode :: Std , SourceType :: InTree , target , Kind :: Clippy ,) ; std_cargo (builder , target , & mut cargo) ; for krate in & * self . crates { cargo . arg ("-p") . arg (krate) ; } let _guard = builder . msg (Kind :: Clippy , format_args ! ("library{}" , crate_description (& self . crates)) , Mode :: Std , build_compiler , target ,) ; run_cargo (builder , cargo , lint_args (builder , & self . config , IGNORED_RULES_FOR_STD_AND_RUSTC) , & build_stamp :: libstd_stamp (builder , build_compiler , target) , vec ! [] , true , false ,) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: clippy ("std" , self . target) . built_by (self . build_compiler)) } }
};
}
