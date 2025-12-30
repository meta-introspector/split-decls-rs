// Generated macro for OffsetName (trait)
macro_rules! Depcrate_timezone_implOffsetName {
() => {
// Module: crate::timezone_impl
// Provides: {"OffsetName"}
// Dependencies: {}
# [doc = " Timezone offset name information."] # [doc = ""] # [doc = " This trait exposes display names that describe an offset in"] # [doc = " various situations."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate chrono;"] # [doc = " # extern crate chrono_tz;"] # [doc = " use chrono::{Duration, Offset, TimeZone};"] # [doc = " use chrono_tz::Europe::London;"] # [doc = " use chrono_tz::OffsetName;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " let london_time = London.ymd(2016, 2, 10).and_hms(12, 0, 0);"] # [doc = " assert_eq!(london_time.offset().tz_id(), \"Europe/London\");"] # [doc = " // London is normally on GMT"] # [doc = " assert_eq!(london_time.offset().abbreviation(), \"GMT\");"] # [doc = ""] # [doc = " let london_summer_time = London.ymd(2016, 5, 10).and_hms(12, 0, 0);"] # [doc = " // The TZ ID remains constant year round"] # [doc = " assert_eq!(london_summer_time.offset().tz_id(), \"Europe/London\");"] # [doc = " // During the summer, this becomes British Summer Time"] # [doc = " assert_eq!(london_summer_time.offset().abbreviation(), \"BST\");"] # [doc = " # }"] # [doc = " ```"] pub trait OffsetName { # [doc = " The IANA TZDB identifier (ex: America/New_York)"] fn tz_id (& self) -> & str ; # [doc = " The abbreviation to use in a longer timestamp (ex: EST)"] # [doc = ""] # [doc = " This takes into account any special offsets that may be in effect."] # [doc = " For example, at a given instant, the time zone with ID *America/New_York*"] # [doc = " may be either *EST* or *EDT*."] fn abbreviation (& self) -> & str ; }
};
}
