// Generated macro for impl_80 (impl)
macro_rules! Depcrate_core_build_steps_cleanimpl_80 {
() => {
// Module: crate::core::build_steps::clean
// Provides: {"impl_80"}
// Dependencies: {}
impl Step for CleanAll { const DEFAULT : bool = true ; type Output = () ; fn make_run (run : RunConfig < '_ >) { run . builder . ensure (CleanAll { }) } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let Subcommand :: Clean { all , stage } = builder . config . cmd else { unreachable ! ("wrong subcommand?") } ; if all && stage . is_some () { panic ! ("--all and --stage can't be used at the same time for `x clean`") ; } clean (builder . build , all , stage) } fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . never () } }
};
}
