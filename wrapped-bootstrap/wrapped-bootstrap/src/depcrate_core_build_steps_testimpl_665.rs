// Generated macro for impl_665 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_665 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_665"}
// Dependencies: {}
impl Step for CrateRustdocJsonTypes { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/rustdoc-json-types") } fn make_run (run : RunConfig < '_ >) { let builder = run . builder ; builder . ensure (CrateRustdocJsonTypes { build_compiler : get_tool_target_compiler (builder , ToolTargetBuildMode :: Build (run . target) ,) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) { let target = self . target ; let cargo = tool :: prepare_tool_cargo (builder , self . build_compiler , Mode :: ToolTarget , target , builder . kind , "src/rustdoc-json-types" , SourceType :: InTree , & [] ,) ; let libtest_args = if target . contains ("musl") { ["'-Ctarget-feature=-crt-static'"] . as_slice () } else { & [] } ; run_cargo_test (cargo , libtest_args , & ["rustdoc-json-types" . to_string ()] , "rustdoc-json-types" , target , builder ,) ; } }
};
}
