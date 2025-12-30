// Generated macro for impl_216 (impl)
macro_rules! Depcrate_datetimeimpl_216 {
() => {
// Module: crate::datetime
// Provides: {"impl_216"}
// Dependencies: {}
impl < Tz : TimeZone , Tz2 : TimeZone > PartialOrd < DateTime < Tz2 > > for DateTime < Tz > { # [doc = " Compare two DateTimes based on their true time, ignoring time zones"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::prelude::*;"] # [doc = ""] # [doc = " let earlier = Utc"] # [doc = "     .with_ymd_and_hms(2015, 5, 15, 2, 0, 0)"] # [doc = "     .unwrap()"] # [doc = "     .with_timezone(&FixedOffset::west_opt(1 * 3600).unwrap());"] # [doc = " let later = Utc"] # [doc = "     .with_ymd_and_hms(2015, 5, 15, 3, 0, 0)"] # [doc = "     .unwrap()"] # [doc = "     .with_timezone(&FixedOffset::west_opt(5 * 3600).unwrap());"] # [doc = ""] # [doc = " assert_eq!(earlier.to_string(), \"2015-05-15 01:00:00 -01:00\");"] # [doc = " assert_eq!(later.to_string(), \"2015-05-14 22:00:00 -05:00\");"] # [doc = ""] # [doc = " assert!(later > earlier);"] # [doc = " ```"] fn partial_cmp (& self , other : & DateTime < Tz2 >) -> Option < Ordering > { self . datetime . partial_cmp (& other . datetime) } }
};
}
