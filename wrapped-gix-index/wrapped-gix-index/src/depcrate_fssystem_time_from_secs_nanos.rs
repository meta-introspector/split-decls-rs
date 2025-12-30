// Generated macro for system_time_from_secs_nanos (function)
macro_rules! Depcrate_fssystem_time_from_secs_nanos {
() => {
// Module: crate::fs
// Provides: {"system_time_from_secs_nanos"}
// Dependencies: {}
# [cfg (not (windows))] fn system_time_from_secs_nanos (secs : i64 , nanos : i32) -> Option < SystemTime > { # [cfg (any (target_os = "macos" , target_os = "ios" , target_os = "tvos" , target_os = "watchos"))] let (secs , nanos) = if (secs <= 0 && secs > i64 :: MIN) && (nanos < 0 && nanos > - 1_000_000_000) { (secs - 1 , nanos + 1_000_000_000) } else { (secs , nanos) } ; let d = std :: time :: Duration :: new (secs . abs_diff (0) , nanos . try_into () . ok () ?) ; Some (if secs < 0 { std :: time :: UNIX_EPOCH - d } else { std :: time :: UNIX_EPOCH + d }) }
};
}
