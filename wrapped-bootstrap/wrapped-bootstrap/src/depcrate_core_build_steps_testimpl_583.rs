// Generated macro for impl_583 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_583 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_583"}
// Dependencies: {}
impl Step for CrateBootstrap { type Output = () ; const IS_HOST : bool = true ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/jsondoclint") . path ("src/tools/replace-version-placeholder") . path ("src/tools/coverage-dump") . alias ("tidyselftest") } fn make_run (run : RunConfig < '_ >) { for path in run . paths { let path = path . assert_single_path () . path . clone () ; run . builder . ensure (CrateBootstrap { host : run . target , path }) ; } } fn run (self , builder : & Builder < '_ >) { let bootstrap_host = builder . config . host_target ; let compiler = builder . compiler (0 , bootstrap_host) ; let mut path = self . path . to_str () . unwrap () ; if path == "tidyselftest" { path = "src/tools/tidy" ; } let cargo = tool :: prepare_tool_cargo (builder , compiler , Mode :: ToolBootstrap , bootstrap_host , Kind :: Test , path , SourceType :: InTree , & [] ,) ; let crate_name = path . rsplit_once ('/') . unwrap () . 1 ; run_cargo_test (cargo , & [] , & [] , crate_name , bootstrap_host , builder) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: test ("crate-bootstrap" , self . host) . with_metadata (self . path . as_path () . to_string_lossy () . to_string ()) ,) } }
};
}
