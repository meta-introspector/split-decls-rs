// Generated macro for DateTime (type)
macro_rules! Depcrate_integrations_jiffDateTime {
() => {
// Module: crate::integrations::jiff
// Provides: {"DateTime"}
// Dependencies: {}
# [doc = " Instant in time represented as the number of nanoseconds since the Unix epoch."] # [doc = ""] # [doc = " Always in UTC."] # [doc = ""] # [doc = " [`DateTime` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`jiff::Timestamp`][2] for details."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/date-time"] # [doc = " [2]: https://docs.rs/jiff/*/jiff/struct.Timestamp.html"] # [graphql_scalar] # [graphql (with = date_time , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/date-time" ,)] pub type DateTime = jiff :: Timestamp ;
};
}
