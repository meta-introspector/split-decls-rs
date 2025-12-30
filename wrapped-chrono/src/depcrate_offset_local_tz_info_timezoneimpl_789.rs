// Generated macro for impl_789 (impl)
macro_rules! Depcrate_offset_local_tz_info_timezoneimpl_789 {
() => {
// Module: crate::offset::local::tz_info::timezone
// Provides: {"impl_789"}
// Dependencies: {}
impl LeapSecond { # [doc = " Construct a TZif file leap second"] pub (super) const fn new (unix_leap_time : i64 , correction : i32) -> Self { Self { unix_leap_time , correction } } # [doc = " Returns Unix leap time"] const fn unix_leap_time (& self) -> i64 { self . unix_leap_time } }
};
}
