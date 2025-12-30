// Generated macro for command (function)
macro_rules! Depcrate_utils_execcommand {
() => {
// Module: crate::utils::exec
// Provides: {"command"}
// Dependencies: {}
# [doc = " Create a new BootstrapCommand. This is a helper function to make command creation"] # [doc = " shorter than `BootstrapCommand::new`."] # [track_caller] # [must_use] pub fn command < S : AsRef < OsStr > > (program : S) -> BootstrapCommand { BootstrapCommand :: new (program) }
};
}
