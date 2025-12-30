// Generated macro for use_152 (pub_use)
macro_rules! Depcrate_sys_signaluse_152 {
() => {
// Module: crate::sys::signal
// Provides: {"use_152"}
// Dependencies: {}
# [cfg (not (any (target_os = "fuchsia" , target_os = "hurd" , target_os = "openbsd" , target_os = "redox")))] # [cfg (any (feature = "aio" , feature = "signal"))] pub use self :: sigevent :: * ;
};
}
