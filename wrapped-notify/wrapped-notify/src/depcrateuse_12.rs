// Generated macro for use_12 (pub_use)
macro_rules! Depcrateuse_12 {
() => {
// Module: crate
// Provides: {"use_12"}
// Dependencies: {}
# [cfg (any (target_os = "freebsd" , target_os = "openbsd" , target_os = "netbsd" , target_os = "dragonfly" , target_os = "ios" , all (target_os = "macos" , feature = "macos_kqueue")))] pub use crate :: kqueue :: KqueueWatcher ;
};
}
