// Generated macro for TimeZoneAbbreviation (enum)
macro_rules! Depcrate_tz_timezoneTimeZoneAbbreviation {
() => {
// Module: crate::tz::timezone
// Provides: {"TimeZoneAbbreviation"}
// Dependencies: {}
# [doc = " A light abstraction over different representations of a time zone"] # [doc = " abbreviation."] # [doc = ""] # [doc = " The lifetime parameter `'t` corresponds to the lifetime of the time zone"] # [doc = " that produced this abbreviation."] # [derive (Clone , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] pub (crate) enum TimeZoneAbbreviation < 't > { # [doc = " For when the abbreviation is borrowed directly from other data. For"] # [doc = " example, from TZif or from POSIX TZ strings."] Borrowed (& 't str) , # [doc = " For when the abbreviation has to be derived from other data. For"] # [doc = " example, from a fixed offset."] # [doc = ""] # [doc = " The idea here is that a `TimeZone` shouldn't need to store the"] # [doc = " string representation of a fixed offset. Particularly in core-only"] # [doc = " environments, this is quite wasteful. So we make the string on-demand"] # [doc = " only when it's requested."] # [doc = ""] # [doc = " An alternative design is to just implement `Display` and reuse"] # [doc = " `Offset`'s `Display` impl, but then we couldn't offer a `-> &str` API."] # [doc = " I feel like that's just a bit overkill, and really just comes from the"] # [doc = " core-only straight-jacket."] Owned (ArrayStr < 9 >) , }
};
}
