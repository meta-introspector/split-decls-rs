// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
# [doc = " Converts from a [`jiff::civil::Time`] to a [`icu_time::Time`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff_icu::{ConvertFrom as _};"] # [doc = ""] # [doc = " let jiff_time = jiff::civil::time(0, 0, 0, 0);"] # [doc = " let icu_time = icu_time::Time::convert_from(jiff_time);"] # [doc = " assert_eq!("] # [doc = "     format!(\"{icu_time:?}\"),"] # [doc = "     \"Time { hour: Hour(0), minute: Minute(0), second: Second(0), subsecond: Nanosecond(0) }\","] # [doc = " );"] # [doc = ""] # [doc = " let jiff_time = jiff::civil::time(17, 59, 4, 0);"] # [doc = " let icu_time = icu_time::Time::convert_from(jiff_time);"] # [doc = " assert_eq!("] # [doc = "     format!(\"{icu_time:?}\"),"] # [doc = "     \"Time { hour: Hour(17), minute: Minute(59), second: Second(4), subsecond: Nanosecond(0) }\","] # [doc = " );"] # [doc = " ```"] # [cfg (feature = "time")] impl ConvertFrom < JiffTime > for IcuTime { fn convert_from (v : JiffTime) -> IcuTime { let hour = v . hour () . unsigned_abs () ; let minute = v . minute () . unsigned_abs () ; let second = v . second () . unsigned_abs () ; let subsec = v . subsec_nanosecond () . unsigned_abs () ; IcuTime :: try_new (hour , minute , second , subsec) . unwrap () } }
};
}
