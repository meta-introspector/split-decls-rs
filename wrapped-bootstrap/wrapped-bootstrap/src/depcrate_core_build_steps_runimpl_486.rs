// Generated macro for impl_486 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_486 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_486"}
// Dependencies: {}
impl Step for ReplaceVersionPlaceholder { type Output = () ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/replace-version-placeholder") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (ReplaceVersionPlaceholder) ; } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let mut cmd = builder . tool_cmd (Tool :: ReplaceVersionPlaceholder) ; cmd . arg (& builder . src) ; cmd . run (builder) ; } }
};
}
