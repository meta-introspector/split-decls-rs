// Generated macro for impl_604 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_604 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_604"}
// Dependencies: {}
impl Step for CompiletestTest { type Output = () ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/compiletest") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (CompiletestTest { host : run . target }) ; } # [doc = " Runs `cargo test` for compiletest."] fn run (self , builder : & Builder < '_ >) { let host = self . host ; if builder . top_stage == 0 && ! builder . config . compiletest_allow_stage0 { eprintln ! ("\
ERROR: `--stage 0` runs compiletest self-tests against the stage0 (precompiled) compiler, not the in-tree compiler, and will almost always cause tests to fail
NOTE: if you're sure you want to do this, please open an issue as to why. In the meantime, you can override this with `--set build.compiletest-allow-stage0=true`.") ; crate :: exit ! (1) ; } let compiler = builder . compiler (builder . top_stage , host) ; debug ! (? compiler) ; builder . std (compiler , host) ; let mut cargo = tool :: prepare_tool_cargo (builder , compiler , Mode :: ToolStd , host , Kind :: Test , "src/tools/compiletest" , SourceType :: InTree , & [] ,) ; cargo . env ("TEST_RUSTC" , builder . rustc (compiler)) ; cargo . allow_features (COMPILETEST_ALLOW_FEATURES) ; run_cargo_test (cargo , & [] , & [] , "compiletest self test" , host , builder) ; } }
};
}
