// Generated macro for impl_623 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_623 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_623"}
// Dependencies: {}
impl Step for CrateBuildHelper { type Output = () ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/build_helper") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (CrateBuildHelper { host : run . target }) ; } # [doc = " Runs `cargo test` for build_helper."] fn run (self , builder : & Builder < '_ >) { let host = self . host ; let compiler = builder . compiler (0 , host) ; let mut cargo = tool :: prepare_tool_cargo (builder , compiler , Mode :: ToolBootstrap , host , Kind :: Test , "src/build_helper" , SourceType :: InTree , & [] ,) ; cargo . allow_features ("test") ; run_cargo_test (cargo , & [] , & [] , "build_helper self test" , host , builder) ; } }
};
}
