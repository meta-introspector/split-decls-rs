// Generated macro for impl_484 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_484 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_484"}
// Dependencies: {}
impl Step for BumpStage0 { type Output = () ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/bump-stage0") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (BumpStage0) ; } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let mut cmd = builder . tool_cmd (Tool :: BumpStage0) ; cmd . args (builder . config . args ()) ; cmd . run (builder) ; } }
};
}
