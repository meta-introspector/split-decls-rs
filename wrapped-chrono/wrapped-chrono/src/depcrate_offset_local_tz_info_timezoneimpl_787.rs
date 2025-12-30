// Generated macro for impl_787 (impl)
macro_rules! Depcrate_offset_local_tz_info_timezoneimpl_787 {
() => {
// Module: crate::offset::local::tz_info::timezone
// Provides: {"impl_787"}
// Dependencies: {}
impl Transition { # [doc = " Construct a TZif file transition"] pub (super) const fn new (unix_leap_time : i64 , local_time_type_index : usize) -> Self { Self { unix_leap_time , local_time_type_index } } # [doc = " Returns Unix leap time"] const fn unix_leap_time (& self) -> i64 { self . unix_leap_time } }
};
}
