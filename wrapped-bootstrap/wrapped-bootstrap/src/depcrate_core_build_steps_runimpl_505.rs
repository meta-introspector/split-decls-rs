// Generated macro for impl_505 (impl)
macro_rules! Depcrate_core_build_steps_runimpl_505 {
() => {
// Module: crate::core::build_steps::run
// Provides: {"impl_505"}
// Dependencies: {}
impl Step for CoverageDump { type Output = () ; const DEFAULT : bool = false ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . path ("src/tools/coverage-dump") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Self { }) ; } fn run (self , builder : & Builder < '_ >) { let mut cmd = builder . tool_cmd (Tool :: CoverageDump) ; cmd . args (& builder . config . free_args) ; cmd . run (builder) ; } }
};
}
