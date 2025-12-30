// Generated macro for impl_1907 (impl)
macro_rules! Depcrate_timeimpl_1907 {
() => {
// Module: crate::time
// Provides: {"impl_1907"}
// Dependencies: {}
impl timespec { pub fn from_usec (microseconds : i64) -> Self { Self { tv_sec : (microseconds / 1_000_000) , tv_nsec : ((microseconds % 1_000_000) * 1000) as i32 , } } pub fn into_usec (& self) -> Option < i64 > { self . tv_sec . checked_mul (1_000_000) . and_then (| usec | usec . checked_add ((self . tv_nsec / 1000) . into ())) } }
};
}
