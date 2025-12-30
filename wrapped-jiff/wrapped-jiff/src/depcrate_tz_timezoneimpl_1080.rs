// Generated macro for impl_1080 (impl)
macro_rules! Depcrate_tz_timezoneimpl_1080 {
() => {
// Module: crate::tz::timezone
// Provides: {"impl_1080"}
// Dependencies: {}
impl < 't > TimeZoneAbbreviation < 't > { # [doc = " Returns this abbreviation as a string borrowed from `self`."] # [doc = ""] # [doc = " Notice that, like `Cow`, the lifetime of the string returned is"] # [doc = " tied to `self` and thus may be shorter than `'t`."] fn as_str < 'a > (& 'a self) -> & 'a str { match * self { TimeZoneAbbreviation :: Borrowed (s) => s , TimeZoneAbbreviation :: Owned (ref s) => s . as_str () , } } }
};
}
