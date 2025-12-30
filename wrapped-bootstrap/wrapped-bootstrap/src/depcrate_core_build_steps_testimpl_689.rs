// Generated macro for impl_689 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_689 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_689"}
// Dependencies: {}
impl Step for TestFloatParse { type Output = () ; const IS_HOST : bool = true ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/test-float-parse") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Self { build_compiler : get_compiler_to_test (run . builder , run . target) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) { let build_compiler = self . build_compiler ; let target = self . target ; builder . std (build_compiler , target) ; builder . std (build_compiler , builder . host_target) ; let mut cargo_test = tool :: prepare_tool_cargo (builder , build_compiler , Mode :: ToolStd , target , Kind :: Test , "src/tools/test-float-parse" , SourceType :: InTree , & [] ,) ; cargo_test . allow_features (TEST_FLOAT_PARSE_ALLOW_FEATURES) ; run_cargo_test (cargo_test , & [] , & [] , "test-float-parse" , target , builder) ; let mut cargo_run = tool :: prepare_tool_cargo (builder , build_compiler , Mode :: ToolStd , target , Kind :: Run , "src/tools/test-float-parse" , SourceType :: InTree , & [] ,) ; cargo_run . allow_features (TEST_FLOAT_PARSE_ALLOW_FEATURES) ; if ! matches ! (env :: var ("FLOAT_PARSE_TESTS_NO_SKIP_HUGE") . as_deref () , Ok ("1") | Ok ("true")) { cargo_run . args (["--" , "--skip-huge"]) ; } cargo_run . into_cmd () . run (builder) ; } }
};
}
