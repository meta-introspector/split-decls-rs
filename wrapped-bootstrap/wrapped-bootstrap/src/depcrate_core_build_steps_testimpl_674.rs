// Generated macro for impl_674 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_674 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_674"}
// Dependencies: {}
impl Step for Bootstrap { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; # [doc = " Tests the build system itself."] fn run (self , builder : & Builder < '_ >) { let host = builder . config . host_target ; let build_compiler = builder . compiler (0 , host) ; builder . build . require_submodule ("src/tools/cargo" , None) ; let mut check_bootstrap = command (builder . python ()) ; check_bootstrap . args (["-m" , "unittest" , "bootstrap_test.py"]) . env ("BUILD_DIR" , & builder . out) . env ("BUILD_PLATFORM" , builder . build . host_target . triple) . env ("BOOTSTRAP_TEST_RUSTC_BIN" , & builder . initial_rustc) . env ("BOOTSTRAP_TEST_CARGO_BIN" , & builder . initial_cargo) . current_dir (builder . src . join ("src/bootstrap/")) ; check_bootstrap . delay_failure () . run (builder) ; let mut cargo = tool :: prepare_tool_cargo (builder , build_compiler , Mode :: ToolBootstrap , host , Kind :: Test , "src/bootstrap" , SourceType :: InTree , & [] ,) ; cargo . release_build (false) ; cargo . rustflag ("-Cdebuginfo=2") . env ("CARGO_TARGET_DIR" , builder . out . join ("bootstrap")) . env ("INSTA_WORKSPACE_ROOT" , & builder . src) . env ("RUSTC_BOOTSTRAP" , "1") ; run_cargo_test (cargo , & [] , & [] , None , host , builder) ; } fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let runs_on_ci = run . builder . config . is_running_on_ci ; run . path ("src/bootstrap") . default_condition (runs_on_ci) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Bootstrap) ; } }
};
}
