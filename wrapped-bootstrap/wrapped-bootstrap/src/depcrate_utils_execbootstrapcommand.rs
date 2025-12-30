// Generated macro for BootstrapCommand (struct)
macro_rules! Depcrate_utils_execBootstrapCommand {
() => {
// Module: crate::utils::exec
// Provides: {"BootstrapCommand"}
// Dependencies: {}
# [doc = " Wrapper around `std::process::Command`."] # [doc = ""] # [doc = " By default, the command will exit bootstrap if it fails."] # [doc = " If you want to allow failures, use [allow_failure]."] # [doc = " If you want to delay failures until the end of bootstrap, use [delay_failure]."] # [doc = ""] # [doc = " By default, the command will print its stdout/stderr to stdout/stderr of bootstrap ([OutputMode::Print])."] # [doc = " If you want to handle the output programmatically, use [BootstrapCommand::run_capture]."] # [doc = ""] # [doc = " Bootstrap will print a debug log to stdout if the command fails and failure is not allowed."] # [doc = ""] # [doc = " By default, command executions are cached based on their workdir, program, arguments, and environment variables."] # [doc = " This avoids re-running identical commands unnecessarily, unless caching is explicitly disabled."] # [doc = ""] # [doc = " [allow_failure]: BootstrapCommand::allow_failure"] # [doc = " [delay_failure]: BootstrapCommand::delay_failure"] pub struct BootstrapCommand { command : Command , pub failure_behavior : BehaviorOnFailure , pub run_in_dry_run : bool , drop_bomb : DropBomb , should_cache : bool , }
};
}
