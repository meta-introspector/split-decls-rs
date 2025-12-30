// Generated macro for NOTIFYING (const)
macro_rules! Depcrate_stateNOTIFYING {
() => {
// Module: crate::state
// Provides: {"NOTIFYING"}
// Dependencies: {}
# [doc = " Set if the awaiter is being notified."] # [doc = ""] # [doc = " This flag is set when notifying the awaiter. If an awaiter is concurrently registered and"] # [doc = " notified, whichever side came first will take over the responsibility of resolving the race."] pub (crate) const NOTIFYING : usize = 1 << 7 ;
};
}
