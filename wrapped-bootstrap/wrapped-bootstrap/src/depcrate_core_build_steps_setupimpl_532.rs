// Generated macro for impl_532 (impl)
macro_rules! Depcrate_core_build_steps_setupimpl_532 {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"impl_532"}
// Dependencies: {}
impl Step for Link { type Output = () ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("link") } fn make_run (run : RunConfig < '_ >) { if run . builder . config . dry_run () { return ; } if let [cmd] = & run . paths [..] && cmd . assert_single_path () . path . as_path () . as_os_str () == "link" { run . builder . ensure (Link) ; } } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let config = & builder . config ; if config . dry_run () { return ; } if ! rustup_installed (builder) { println ! ("WARNING: `rustup` is not installed; Skipping `stage1` toolchain linking.") ; return ; } let stage_path = ["build" , config . host_target . rustc_target_arg () , "stage1"] . join (MAIN_SEPARATOR_STR) ; if stage_dir_exists (& stage_path [..]) && ! config . dry_run () { attempt_toolchain_link (builder , & stage_path [..]) ; } } }
};
}
