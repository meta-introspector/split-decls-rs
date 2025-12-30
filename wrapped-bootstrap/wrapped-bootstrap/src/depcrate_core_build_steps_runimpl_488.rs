// Generated macro for impl_488 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_488 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_488"}
// Dependencies: {}
impl Step for Miri { type Output = () ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/miri") } fn make_run (run : RunConfig < '_ >) { let builder = run . builder ; let stage = if builder . config . is_explicit_stage () || builder . top_stage >= 1 { builder . top_stage } else { 1 } ; if stage == 0 { eprintln ! ("ERROR: miri cannot be run at stage 0") ; exit ! (1) ; } let compilers = RustcPrivateCompilers :: new (builder , stage , builder . host_target) ; run . builder . ensure (Miri { compilers , target : run . target }) ; } fn run (self , builder : & Builder < '_ >) { let host = builder . build . host_target ; let compilers = self . compilers ; let target = self . target ; builder . ensure (tool :: Miri :: from_compilers (compilers)) ; let miri_sysroot = test :: Miri :: build_miri_sysroot (builder , compilers . target_compiler () , target) ; let mut miri = tool :: prepare_tool_cargo (builder , compilers . build_compiler () , Mode :: ToolRustcPrivate , host , Kind :: Run , "src/tools/miri" , SourceType :: InTree , & [] ,) ; miri . add_rustc_lib_path (builder) ; miri . arg ("--") . arg ("--target") . arg (target . rustc_target_arg ()) ; miri . arg ("--sysroot") . arg (miri_sysroot) ; miri . args (builder . config . args ()) ; miri . into_cmd () . run (builder) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: run ("miri" , self . target) . built_by (self . compilers . build_compiler ())) } }
};
}
