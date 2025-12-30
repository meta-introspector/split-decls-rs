// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
# [doc = " Converts from a [`icu_time::Time`] to a [`jiff::civil::Time`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff_icu::{ConvertTryFrom as _};"] # [doc = ""] # [doc = " let icu_time = icu_time::Time::try_new(0, 0, 0, 0).unwrap();"] # [doc = " let jiff_time = jiff::civil::Time::convert_try_from(icu_time)?;"] # [doc = " assert_eq!(jiff_time, jiff::civil::time(0, 0, 0, 0));"] # [doc = ""] # [doc = " let icu_time = icu_time::Time::try_new(23, 59, 59, 999_999_999).unwrap();"] # [doc = " let jiff_time = jiff::civil::Time::convert_try_from(icu_time)?;"] # [doc = " assert_eq!(jiff_time, jiff::civil::time(23, 59, 59, 999_999_999));"] # [doc = ""] # [doc = " let icu_time = icu_time::Time::try_new(17, 59, 4, 0).unwrap();"] # [doc = " let jiff_time = jiff::civil::Time::convert_try_from(icu_time)?;"] # [doc = " assert_eq!(jiff_time, jiff::civil::time(17, 59, 4, 0));"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [cfg (feature = "time")] impl ConvertTryFrom < IcuTime > for JiffTime { type Error = Error ; fn convert_try_from (v : IcuTime) -> Result < JiffTime , Error > { let mut hour = i8 :: try_from (v . hour . number ()) . unwrap () ; if hour == 24 { hour = 0 ; } let minute = i8 :: try_from (v . minute . number ()) . unwrap () ; let second = i8 :: try_from (v . second . number ()) . unwrap () ; let subsec_nano = i32 :: try_from (v . subsecond . number ()) . unwrap () ; Ok (JiffTime :: new (hour , minute , second , subsec_nano) ?) } }
};
}
