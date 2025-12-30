// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
# [doc = " Converts from a [`icu_calendar::types::Weekday`] to a"] # [doc = " [`jiff::civil::Weekday`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff_icu::{ConvertFrom as _};"] # [doc = ""] # [doc = " let icu_weekday = icu_calendar::types::Weekday::Wednesday;"] # [doc = " let jiff_weekday = jiff::civil::Weekday::convert_from(icu_weekday);"] # [doc = " assert_eq!(jiff_weekday, jiff::civil::Weekday::Wednesday);"] # [doc = " ```"] impl ConvertFrom < IcuWeekday > for JiffWeekday { fn convert_from (v : IcuWeekday) -> JiffWeekday { match v { IcuWeekday :: Monday => JiffWeekday :: Monday , IcuWeekday :: Tuesday => JiffWeekday :: Tuesday , IcuWeekday :: Wednesday => JiffWeekday :: Wednesday , IcuWeekday :: Thursday => JiffWeekday :: Thursday , IcuWeekday :: Friday => JiffWeekday :: Friday , IcuWeekday :: Saturday => JiffWeekday :: Saturday , IcuWeekday :: Sunday => JiffWeekday :: Sunday , } } }
};
}
