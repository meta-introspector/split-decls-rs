// Generated macro for ProcessCommand (struct)
macro_rules! Depcrate_cliProcessCommand {
() => {
// Module: crate::cli
// Provides: {"ProcessCommand"}
// Dependencies: {}
# [derive (Args , Debug)] struct ProcessCommand { # [command (flatten)] target_args : TargetArgs , # [doc = " Limits the operation to one or more snapshots."] # [arg (long = "snapshot")] snapshot_filter : Option < Vec < String > > , # [doc = " Do not print to stdout."] # [arg (short = 'q' , long)] quiet : bool , }
};
}
