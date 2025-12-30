// Generated macro for TimeZoneOrUtcOffset (type)
macro_rules! Depcrate_integrations_jiffTimeZoneOrUtcOffset {
() => {
// Module: crate::integrations::jiff
// Provides: {"TimeZoneOrUtcOffset"}
// Dependencies: {}
# [doc = " Representation of a time zone or UTC offset."] # [doc = ""] # [doc = " Can be one of three possible representations:"] # [doc = " - Identifier from the [IANA Time Zone Database][0]."] # [doc = " - Fixed offset from UTC (`±hh:mm`)."] # [doc = ""] # [doc = " May be seen as a combination of both [`TimeZone`][3] and [`UtcOffset` scalars][4]."] # [doc = ""] # [doc = " See also [`jiff::tz::TimeZone`][2] for details."] # [doc = ""] # [doc = " [0]: http://iana.org/time-zones"] # [doc = " [2]: https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html"] # [doc = " [3]: https://graphql-scalars.dev/docs/scalars/time-zone"] # [doc = " [4]: https://graphql-scalars.dev/docs/scalars/utc-offset"] # [graphql_scalar] # [graphql (with = time_zone_or_utc_offset , parse_token (String) ,)] pub type TimeZoneOrUtcOffset = jiff :: tz :: TimeZone ;
};
}
