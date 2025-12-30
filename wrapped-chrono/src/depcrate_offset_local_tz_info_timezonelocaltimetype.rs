// Generated macro for LocalTimeType (struct)
macro_rules! Depcrate_offset_local_tz_info_timezoneLocalTimeType {
() => {
// Module: crate::offset::local::tz_info::timezone
// Provides: {"LocalTimeType"}
// Dependencies: {}
# [doc = " Local time type associated to a time zone"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (crate) struct LocalTimeType { # [doc = " Offset from UTC in seconds"] pub (super) ut_offset : i32 , # [doc = " Daylight Saving Time indicator"] is_dst : bool , # [doc = " Time zone name"] name : Option < TimeZoneName > , }
};
}
