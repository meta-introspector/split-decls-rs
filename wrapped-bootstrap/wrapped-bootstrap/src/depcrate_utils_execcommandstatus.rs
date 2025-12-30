// Generated macro for CommandStatus (enum)
macro_rules! Depcrate_utils_execCommandStatus {
() => {
// Module: crate::utils::exec
// Provides: {"CommandStatus"}
// Dependencies: {}
# [doc = " Represents the current status of `BootstrapCommand`."] # [derive (Clone , PartialEq)] enum CommandStatus { # [doc = " The command has started and finished with some status."] Finished (ExitStatus) , # [doc = " It was not even possible to start the command or wait for it to finish."] DidNotStartOrFinish , }
};
}
