// Generated macro for DateTime (type)
macro_rules! Depcrate_integrations_chronoDateTime {
() => {
// Module: crate::integrations::chrono
// Provides: {"DateTime"}
// Dependencies: {}
# [doc = " Combined date and time (with time zone) in [RFC 3339][0] format."] # [doc = ""] # [doc = " Represents a description of an exact instant on the time-line (such as the"] # [doc = " instant that a user account was created)."] # [doc = ""] # [doc = " [`DateTime` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`chrono::DateTime`][2] for details."] # [doc = ""] # [doc = " [0]: https://datatracker.ietf.org/doc/html/rfc3339#section-5"] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/date-time"] # [doc = " [2]: https://docs.rs/chrono/latest/chrono/struct.DateTime.html"] # [graphql_scalar] # [graphql (with = date_time , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/date-time" , where (Tz : TimeZone + FromFixedOffset , Tz :: Offset : fmt :: Display ,))] pub type DateTime < Tz > = chrono :: DateTime < Tz > ;
};
}
