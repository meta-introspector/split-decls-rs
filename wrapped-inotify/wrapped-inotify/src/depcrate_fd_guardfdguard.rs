// Generated macro for FdGuard (struct)
macro_rules! Depcrate_fd_guardFdGuard {
() => {
// Module: crate::fd_guard
// Provides: {"FdGuard"}
// Dependencies: {}
# [doc = " A RAII guard around a `RawFd` that closes it automatically on drop."] # [derive (Debug)] pub struct FdGuard { pub (crate) fd : RawFd , pub (crate) close_on_drop : AtomicBool , }
};
}
