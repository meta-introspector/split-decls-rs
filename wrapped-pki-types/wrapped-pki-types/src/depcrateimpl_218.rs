// Generated macro for impl_218 (impl)
macro_rules! Depcrateimpl_218 {
() => {
// Module: crate
// Provides: {"impl_218"}
// Dependencies: {}
impl UnixTime { # [doc = " The current time, as a `UnixTime`"] # [cfg (any (all (feature = "std" , not (all (target_family = "wasm" , target_os = "unknown"))) , all (target_family = "wasm" , target_os = "unknown" , feature = "web")))] pub fn now () -> Self { Self :: since_unix_epoch (SystemTime :: now () . duration_since (SystemTime :: UNIX_EPOCH) . unwrap () ,) } # [doc = " Convert a `Duration` since the start of 1970 to a `UnixTime`"] # [doc = ""] # [doc = " The `duration` must be relative to the Unix epoch."] pub const fn since_unix_epoch (duration : Duration) -> Self { Self (duration . as_secs ()) } # [doc = " Number of seconds since the Unix epoch"] pub const fn as_secs (& self) -> u64 { self . 0 } }
};
}
