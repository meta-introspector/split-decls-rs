// Generated macro for impl_543 (impl)
macro_rules! Depcrate_core_build_steps_setupimpl_543 {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"impl_543"}
// Dependencies: {}
impl Step for Hook { type Output = () ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("hook") } fn make_run (run : RunConfig < '_ >) { if let [cmd] = & run . paths [..] && cmd . assert_single_path () . path . as_path () . as_os_str () == "hook" { run . builder . ensure (Hook) ; } } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let config = & builder . config ; if config . dry_run () || ! config . rust_info . is_managed_git_subrepository () { return ; } t ! (install_git_hook_maybe (builder , config)) ; } }
};
}
