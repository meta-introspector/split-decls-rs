// Generated macro for LocalTime (type)
macro_rules! Depcrate_integrations_jiffLocalTime {
() => {
// Module: crate::integrations::jiff
// Provides: {"LocalTime"}
// Dependencies: {}
# [doc = " Representation of a civil \"wall clock\" time."] # [doc = ""] # [doc = " Conceptually, corresponds to the typical hours and minutes that you might see on a clock. This"] # [doc = " type also contains the second and fractional subsecond (to nanosecond precision) associated with"] # [doc = " a time."] # [doc = ""] # [doc = " [`LocalTime` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`jiff::civil::Time`][2] for details."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-time"] # [doc = " [2]: https://docs.rs/jiff/*/jiff/civil/struct.Time.html"] # [graphql_scalar] # [graphql (with = local_time , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/local-time" ,)] pub type LocalTime = jiff :: civil :: Time ;
};
}
