// Generated macro for TimeZone (struct)
macro_rules! Depcrate_integrations_jiffTimeZone {
() => {
// Module: crate::integrations::jiff
// Provides: {"TimeZone"}
// Dependencies: {}
# [doc = " Representation of a time zone from the [IANA Time Zone Database][0]."] # [doc = ""] # [doc = " A set of rules for determining the civil time, via an offset from UTC, in a particular"] # [doc = " geographic region. In many cases, the offset in a particular time zone can vary over the course"] # [doc = " of a year through transitions into and out of daylight saving time."] # [doc = ""] # [doc = " [`TimeZone` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`jiff::tz::TimeZone`][2] for details."] # [doc = ""] # [doc = " [0]: http://iana.org/time-zones"] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/time-zone"] # [doc = " [2]: https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html"] # [graphql_scalar] # [graphql (with = time_zone , to_output_with = ScalarValue :: from_displayable , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/time-zone" ,)] # [derive (Clone , Debug , Display , Eq , Into , PartialEq)] # [display ("{}" , _0 . iana_name () . expect ("failed to display `TimeZone`: no IANA name"))] pub struct TimeZone (jiff :: tz :: TimeZone) ;
};
}
