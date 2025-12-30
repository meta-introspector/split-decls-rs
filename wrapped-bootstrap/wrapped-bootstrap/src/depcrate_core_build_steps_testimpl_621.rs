// Generated macro for impl_621 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_621 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_621"}
// Dependencies: {}
impl Step for CrateRunMakeSupport { type Output = () ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/run-make-support") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (CrateRunMakeSupport { host : run . target }) ; } # [doc = " Runs `cargo test` for run-make-support."] fn run (self , builder : & Builder < '_ >) { let host = self . host ; let compiler = builder . compiler (0 , host) ; let mut cargo = tool :: prepare_tool_cargo (builder , compiler , Mode :: ToolBootstrap , host , Kind :: Test , "src/tools/run-make-support" , SourceType :: InTree , & [] ,) ; cargo . allow_features ("test") ; run_cargo_test (cargo , & [] , & [] , "run-make-support self test" , host , builder) ; } }
};
}
