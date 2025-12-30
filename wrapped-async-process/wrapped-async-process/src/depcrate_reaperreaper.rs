// Generated macro for Reaper (enum)
macro_rules! Depcrate_reaperReaper {
() => {
// Module: crate::reaper
// Provides: {"Reaper"}
// Dependencies: {}
# [doc = " The underlying system reaper."] pub (crate) enum Reaper { # [cfg (any (windows , target_os = "linux"))] # [doc = " The reaper based on the wait backend."] Wait (wait :: Reaper) , # [doc = " The reaper based on the signal backend."] # [cfg (not (windows))] Signal (signal :: Reaper) , }
};
}
