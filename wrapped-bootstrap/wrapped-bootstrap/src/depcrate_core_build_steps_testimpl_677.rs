// Generated macro for impl_677 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_677 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_677"}
// Dependencies: {}
impl Step for TierCheck { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/tier-check") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (TierCheck { test_compiler : get_compiler_to_test (run . builder , run . target) }) ; } fn run (self , builder : & Builder < '_ >) { let tool_build_compiler = builder . compiler (0 , builder . host_target) ; let mut cargo = tool :: prepare_tool_cargo (builder , tool_build_compiler , Mode :: ToolBootstrap , tool_build_compiler . host , Kind :: Run , "src/tools/tier-check" , SourceType :: InTree , & [] ,) ; cargo . arg (builder . src . join ("src/doc/rustc/src/platform-support.md")) ; cargo . arg (builder . rustc (self . test_compiler)) ; if builder . is_verbose () { cargo . arg ("--verbose") ; } let _guard = builder . msg_test ("platform support check" , self . test_compiler . host , self . test_compiler . stage ,) ; BootstrapCommand :: from (cargo) . delay_failure () . run (builder) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: test ("tier-check" , self . test_compiler . host)) } }
};
}
