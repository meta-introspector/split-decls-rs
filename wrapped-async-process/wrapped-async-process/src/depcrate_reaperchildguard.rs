// Generated macro for ChildGuard (enum)
macro_rules! Depcrate_reaperChildGuard {
() => {
// Module: crate::reaper
// Provides: {"ChildGuard"}
// Dependencies: {}
# [doc = " The wrapper around a child."] pub (crate) enum ChildGuard { # [cfg (any (windows , target_os = "linux"))] # [doc = " The child guard based on the wait backend."] Wait (wait :: ChildGuard) , # [doc = " The child guard based on the signal backend."] # [cfg (not (windows))] Signal (signal :: ChildGuard) , }
};
}
