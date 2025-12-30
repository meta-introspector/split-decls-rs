// Generated macro for std_time_to_c (function)
macro_rules! Depcrate_ffistd_time_to_c {
() => {
// Module: crate::ffi
// Provides: {"std_time_to_c"}
// Dependencies: {}
# [cfg (any (target_os = "macos" , target_os = "ios" , target_os = "windows"))] fn std_time_to_c (_time : & Instant , out : & mut timespec) { out . tv_sec = 0 ; out . tv_nsec = 0 ; }
};
}
