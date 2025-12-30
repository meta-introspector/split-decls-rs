// Generated macro for Lock (enum)
macro_rules! Depcrate_reaperLock {
() => {
// Module: crate::reaper
// Provides: {"Lock"}
// Dependencies: {}
# [doc = " A lock on the reaper."] pub (crate) enum Lock { # [cfg (any (windows , target_os = "linux"))] # [doc = " The wait-based reaper needs no lock."] Wait , # [doc = " The lock for the signal-based reaper."] # [cfg (not (windows))] Signal (signal :: Lock) , }
};
}
