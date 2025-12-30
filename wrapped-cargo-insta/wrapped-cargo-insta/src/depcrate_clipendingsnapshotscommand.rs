// Generated macro for PendingSnapshotsCommand (struct)
macro_rules! Depcrate_cliPendingSnapshotsCommand {
() => {
// Module: crate::cli
// Provides: {"PendingSnapshotsCommand"}
// Dependencies: {}
# [derive (Args , Debug)] # [command (rename_all = "kebab-case")] struct PendingSnapshotsCommand { # [command (flatten)] target_args : TargetArgs , # [doc = " Changes the output from human readable to JSON."] # [arg (long)] as_json : bool , }
};
}
