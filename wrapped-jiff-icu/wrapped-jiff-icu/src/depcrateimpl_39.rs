// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
# [doc = " Converts from a [`jiff::civil::Weekday`] to a"] # [doc = " [`icu_calendar::types::Weekday`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff_icu::{ConvertFrom as _};"] # [doc = ""] # [doc = " let jiff_weekday = jiff::civil::Weekday::Wednesday;"] # [doc = " let icu_weekday = icu_calendar::types::Weekday::convert_from(jiff_weekday);"] # [doc = " assert_eq!(icu_weekday, icu_calendar::types::Weekday::Wednesday);"] # [doc = " ```"] impl ConvertFrom < JiffWeekday > for IcuWeekday { fn convert_from (v : JiffWeekday) -> IcuWeekday { match v { JiffWeekday :: Monday => IcuWeekday :: Monday , JiffWeekday :: Tuesday => IcuWeekday :: Tuesday , JiffWeekday :: Wednesday => IcuWeekday :: Wednesday , JiffWeekday :: Thursday => IcuWeekday :: Thursday , JiffWeekday :: Friday => IcuWeekday :: Friday , JiffWeekday :: Saturday => IcuWeekday :: Saturday , JiffWeekday :: Sunday => IcuWeekday :: Sunday , } } }
};
}
