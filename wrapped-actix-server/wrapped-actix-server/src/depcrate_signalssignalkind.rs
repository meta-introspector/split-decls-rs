// Generated macro for SignalKind (enum)
macro_rules! Depcrate_signalsSignalKind {
() => {
// Module: crate::signals
// Provides: {"SignalKind"}
// Dependencies: {}
# [doc = " Types of process signals."] # [derive (Debug , Clone , Copy , PartialEq)] # [allow (dead_code)] pub (crate) enum SignalKind { # [doc = " Cancellation token or channel."] Cancel , # [doc = " OS `SIGINT`."] OsInt , # [doc = " OS `SIGTERM`."] OsTerm , # [doc = " OS `SIGQUIT`."] OsQuit , }
};
}
