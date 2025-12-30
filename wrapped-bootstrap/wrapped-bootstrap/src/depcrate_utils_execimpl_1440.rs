// Generated macro for impl_1440 (impl)
macro_rules! Depcrate_utils_execimpl_1440 {
() => {
// Module: crate::utils::exec
// Provides: {"impl_1440"}
// Dependencies: {}
impl From < Command > for BootstrapCommand { # [track_caller] fn from (command : Command) -> Self { let program = command . get_program () . to_owned () ; Self { should_cache : false , command , failure_behavior : BehaviorOnFailure :: Exit , run_in_dry_run : false , drop_bomb : DropBomb :: arm (program) , } } }
};
}
