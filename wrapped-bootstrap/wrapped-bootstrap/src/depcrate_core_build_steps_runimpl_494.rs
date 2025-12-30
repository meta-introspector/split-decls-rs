// Generated macro for impl_494 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_494 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_494"}
// Dependencies: {}
impl Step for GenerateWindowsSys { type Output = () ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/generate-windows-sys") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (GenerateWindowsSys) ; } fn run (self , builder : & Builder < '_ >) { let mut cmd = builder . tool_cmd (Tool :: GenerateWindowsSys) ; cmd . arg (& builder . src) ; cmd . run (builder) ; } }
};
}
