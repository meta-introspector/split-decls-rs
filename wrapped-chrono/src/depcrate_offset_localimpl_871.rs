// Generated macro for impl_871 (impl)
macro_rules! Depcrate_offset_localimpl_871 {
() => {
// Module: crate::offset::local
// Provides: {"impl_871"}
// Dependencies: {}
impl Local { # [doc = " Returns a `Date` which corresponds to the current date."] # [deprecated (since = "0.4.23" , note = "use `Local::now()` instead")] # [allow (deprecated)] # [must_use] pub fn today () -> Date < Local > { Local :: now () . date () } # [doc = " Returns a `DateTime<Local>` which corresponds to the current date, time and offset from"] # [doc = " UTC."] # [doc = ""] # [doc = " See also the similar [`Utc::now()`] which returns `DateTime<Utc>`, i.e. without the local"] # [doc = " offset."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #![allow(unused_variables)]"] # [doc = " # use chrono::{DateTime, FixedOffset, Local};"] # [doc = " // Current local time"] # [doc = " let now = Local::now();"] # [doc = ""] # [doc = " // Current local date"] # [doc = " let today = now.date_naive();"] # [doc = ""] # [doc = " // Current local time, converted to `DateTime<FixedOffset>`"] # [doc = " let now_fixed_offset = Local::now().fixed_offset();"] # [doc = " // or"] # [doc = " let now_fixed_offset: DateTime<FixedOffset> = Local::now().into();"] # [doc = ""] # [doc = " // Current time in some timezone (let's use +05:00)"] # [doc = " // Note that it is usually more efficient to use `Utc::now` for this use case."] # [doc = " let offset = FixedOffset::east_opt(5 * 60 * 60).unwrap();"] # [doc = " let now_with_offset = Local::now().with_timezone(&offset);"] # [doc = " ```"] pub fn now () -> DateTime < Local > { Utc :: now () . with_timezone (& Local) } }
};
}
