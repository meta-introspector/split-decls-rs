// Generated macro for Date (struct)
macro_rules! Depcrate_common_dateDate {
() => {
// Module: crate::common::date
// Provides: {"Date"}
// Dependencies: {}
# [doc = " `Date` header, defined in [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.1.2)"] # [doc = ""] # [doc = " The `Date` header field represents the date and time at which the"] # [doc = " message was originated."] # [doc = ""] # [doc = " ## ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Date = HTTP-date"] # [doc = " ```"] # [doc = ""] # [doc = " ## Example values"] # [doc = ""] # [doc = " * `Tue, 15 Nov 1994 08:12:31 GMT`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Date;"] # [doc = " use std::time::SystemTime;"] # [doc = ""] # [doc = " let date = Date::from(SystemTime::now());"] # [doc = " ```"] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Date (HttpDate) ;
};
}
