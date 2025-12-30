// Generated macro for unavailable (module)
macro_rules! Depcrate_backendsunavailable {
() => {
// Module: crate::backends
// Provides: {"unavailable"}
// Dependencies: {}
# [cfg (not (any (target_os = "android" , target_os = "ios" , target_os = "linux" , target_os = "macos" , target_os = "windows")))] # [doc (hidden)] mod unavailable ;
};
}
