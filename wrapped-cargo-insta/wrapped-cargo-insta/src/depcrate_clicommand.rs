// Generated macro for Command (enum)
macro_rules! Depcrate_cliCommand {
() => {
// Module: crate::cli
// Provides: {"Command"}
// Dependencies: {}
# [derive (Subcommand , Debug)] # [command (version , after_help = "For the online documentation of the latest version, see https://insta.rs/docs/cli/.")] # [allow (clippy :: large_enum_variant)] enum Command { # [doc = " Interactively review snapshots"] # [command (alias = "verify")] Review (ProcessCommand) , # [doc = " Rejects all snapshots"] Reject (ProcessCommand) , # [doc = " Accept all snapshots"] # [command (alias = "approve")] Accept (ProcessCommand) , # [doc = " Run tests and then reviews"] Test (TestCommand) , # [doc = " Print a summary of all pending snapshots."] PendingSnapshots (PendingSnapshotsCommand) , # [doc = " Shows a specific snapshot"] Show (ShowCommand) , }
};
}
