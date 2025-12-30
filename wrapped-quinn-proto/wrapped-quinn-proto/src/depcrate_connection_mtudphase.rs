// Generated macro for Phase (enum)
macro_rules! Depcrate_connection_mtudPhase {
() => {
// Module: crate::connection::mtud
// Provides: {"Phase"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] enum Phase { # [doc = " We haven't started polling yet"] Initial , # [doc = " We are currently searching for a higher PMTU"] Searching (SearchState) , # [doc = " Searching has completed and will be triggered again at the provided instant"] Complete (Instant) , }
};
}
