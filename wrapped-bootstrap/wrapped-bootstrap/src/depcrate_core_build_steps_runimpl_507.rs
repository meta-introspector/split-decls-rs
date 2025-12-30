// Generated macro for impl_507 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_507 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_507"}
// Dependencies: {}
impl Step for Rustfmt { type Output = () ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/rustfmt") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Rustfmt) ; } fn run (self , builder : & Builder < '_ >) { let host = builder . build . host_target ; let stage = if builder . config . is_explicit_stage () || builder . top_stage >= 1 { builder . top_stage } else { 1 } ; if stage == 0 { eprintln ! ("rustfmt cannot be run at stage 0") ; eprintln ! ("HELP: Use `x fmt` to use stage 0 rustfmt.") ; std :: process :: exit (1) ; } let compilers = RustcPrivateCompilers :: new (builder , stage , host) ; let rustfmt_build = builder . ensure (tool :: Rustfmt :: from_compilers (compilers)) ; let mut rustfmt = tool :: prepare_tool_cargo (builder , rustfmt_build . build_compiler , Mode :: ToolRustcPrivate , host , Kind :: Run , "src/tools/rustfmt" , SourceType :: InTree , & [] ,) ; rustfmt . args (["--bin" , "rustfmt" , "--"]) ; rustfmt . args (builder . config . args ()) ; rustfmt . into_cmd () . run (builder) ; } }
};
}
