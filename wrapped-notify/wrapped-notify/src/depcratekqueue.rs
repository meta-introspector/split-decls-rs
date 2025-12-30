// Generated macro for kqueue (module)
macro_rules! Depcratekqueue {
() => {
// Module: crate
// Provides: {"kqueue"}
// Dependencies: {}
# [cfg (any (target_os = "freebsd" , target_os = "openbsd" , target_os = "dragonfly" , target_os = "netbsd" , target_os = "ios" , all (target_os = "macos" , feature = "macos_kqueue")))] pub mod kqueue ;
};
}
