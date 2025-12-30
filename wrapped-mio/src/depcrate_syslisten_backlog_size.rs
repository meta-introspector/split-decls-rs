// Generated macro for LISTEN_BACKLOG_SIZE (const)
macro_rules! Depcrate_sysLISTEN_BACKLOG_SIZE {
() => {
// Module: crate::sys
// Provides: {"LISTEN_BACKLOG_SIZE"}
// Dependencies: {}
# [allow (dead_code)] # [cfg (not (any (target_os = "windows" , target_os = "redox" , target_os = "espidf" , target_os = "horizon" , target_os = "linux" , target_os = "freebsd" , target_os = "openbsd" , target_os = "wasi" , target_os = "hermit" , target_vendor = "apple" ,)))] pub (crate) const LISTEN_BACKLOG_SIZE : i32 = libc :: SOMAXCONN ;
};
}
