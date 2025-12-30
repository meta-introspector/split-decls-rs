// Generated macro for ChildGuard (struct)
macro_rules! DepcrateChildGuard {
() => {
// Module: crate
// Provides: {"ChildGuard"}
// Dependencies: {}
# [doc = " A guard that can kill child processes, or push them into the zombie list."] struct ChildGuard { inner : reaper :: ChildGuard , reap_on_drop : bool , kill_on_drop : bool , reaper : & 'static Reaper , }
};
}
