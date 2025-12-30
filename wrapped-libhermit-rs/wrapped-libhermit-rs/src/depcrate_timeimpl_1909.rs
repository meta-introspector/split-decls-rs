// Generated macro for impl_1909 (impl)
macro_rules! Depcrate_timeimpl_1909 {
() => {
// Module: crate::time
// Provides: {"impl_1909"}
// Dependencies: {}
impl SystemTime { pub const UNIX_EPOCH : SystemTime = Self (timespec { tv_sec : 0 , tv_nsec : 0 , }) ; # [doc = " Returns the system time corresponding to \"now\"."] pub fn now () -> Self { Self (timespec :: from_usec (arch :: kernel :: systemtime :: now_micros () as i64)) } # [doc = " Returns the amount of time elapsed from an earlier point in time."] pub fn duration_since (& self , earlier : SystemTime) -> Duration { Duration :: from_micros (self . 0 . into_usec () . unwrap () . checked_sub (earlier . 0 . into_usec () . unwrap ()) . unwrap () . try_into () . unwrap () ,) } }
};
}
