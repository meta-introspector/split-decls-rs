// Generated macro for impl_562 (impl)
macro_rules! Depcrate_naive_datetimeimpl_562 {
() => {
// Module: crate::naive::datetime
// Provides: {"impl_562"}
// Dependencies: {}
impl From < NaiveDate > for NaiveDateTime { # [doc = " Converts a `NaiveDate` to a `NaiveDateTime` of the same date but at midnight."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{NaiveDate, NaiveDateTime};"] # [doc = ""] # [doc = " let nd = NaiveDate::from_ymd_opt(2016, 5, 28).unwrap();"] # [doc = " let ndt = NaiveDate::from_ymd_opt(2016, 5, 28).unwrap().and_hms_opt(0, 0, 0).unwrap();"] # [doc = " assert_eq!(ndt, NaiveDateTime::from(nd));"] fn from (date : NaiveDate) -> Self { date . and_hms_opt (0 , 0 , 0) . unwrap () } }
};
}
