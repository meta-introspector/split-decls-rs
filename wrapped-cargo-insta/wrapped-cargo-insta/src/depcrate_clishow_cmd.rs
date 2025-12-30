// Generated macro for show_cmd (function)
macro_rules! Depcrate_clishow_cmd {
() => {
// Module: crate::cli
// Provides: {"show_cmd"}
// Dependencies: {}
fn show_cmd (cmd : ShowCommand) -> Result < () , Box < dyn Error > > { let loc = handle_target_args (& cmd . target_args , & []) ? ; let snapshot = Snapshot :: from_file (& cmd . path) ? ; let mut printer = SnapshotPrinter :: new (& loc . workspace_root , None , & snapshot) ; printer . set_snapshot_file (Some (& cmd . path)) ; printer . set_show_info (true) ; printer . set_show_diff (false) ; printer . print () ; Ok (()) }
};
}
