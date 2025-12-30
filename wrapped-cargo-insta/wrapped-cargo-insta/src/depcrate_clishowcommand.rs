// Generated macro for ShowCommand (struct)
macro_rules! Depcrate_cliShowCommand {
() => {
// Module: crate::cli
// Provides: {"ShowCommand"}
// Dependencies: {}
# [derive (Args , Debug)] # [command (rename_all = "kebab-case")] struct ShowCommand { # [command (flatten)] target_args : TargetArgs , # [doc = " The path to the snapshot file."] path : PathBuf , }
};
}
