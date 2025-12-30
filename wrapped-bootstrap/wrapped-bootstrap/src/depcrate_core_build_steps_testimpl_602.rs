// Generated macro for impl_602 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_602 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_602"}
// Dependencies: {}
impl Step for CargoMiri { type Output = () ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/miri/cargo-miri") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (CargoMiri { target : run . target }) ; } # [doc = " Tests `cargo miri test`."] fn run (self , builder : & Builder < '_ >) { let host = builder . build . host_target ; let target = self . target ; let stage = builder . top_stage ; if stage == 0 { eprintln ! ("cargo-miri cannot be tested at stage 0") ; std :: process :: exit (1) ; } let build_compiler = builder . compiler (stage , host) ; let mut cargo = tool :: prepare_tool_cargo (builder , build_compiler , Mode :: ToolStd , target , Kind :: MiriTest , "src/tools/miri/test-cargo-miri" , SourceType :: Submodule , & [] ,) ; match builder . doc_tests { DocTests :: Yes => { } DocTests :: No => { cargo . args (["--lib" , "--bins" , "--examples" , "--tests" , "--benches"]) ; } DocTests :: Only => { cargo . arg ("--doc") ; } } cargo . arg ("--") . args (builder . config . test_args ()) ; let mut cargo = BootstrapCommand :: from (cargo) ; { let _guard = builder . msg_test ("cargo-miri" , target , stage) ; let _time = helpers :: timeit (builder) ; cargo . run (builder) ; } } }
};
}
