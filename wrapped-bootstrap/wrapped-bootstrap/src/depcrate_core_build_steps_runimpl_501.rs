// Generated macro for impl_501 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_501 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_501"}
// Dependencies: {}
impl Step for FeaturesStatusDump { type Output = () ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/features-status-dump") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (FeaturesStatusDump) ; } fn run (self , builder : & Builder < '_ >) { let mut cmd = builder . tool_cmd (Tool :: FeaturesStatusDump) ; cmd . arg ("--library-path") ; cmd . arg (builder . src . join ("library")) ; cmd . arg ("--compiler-path") ; cmd . arg (builder . src . join ("compiler")) ; cmd . arg ("--output-path") ; cmd . arg (builder . out . join ("features-status-dump.json")) ; cmd . run (builder) ; } }
};
}
