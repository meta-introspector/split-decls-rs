// Generated macro for LocalDateTime (type)
macro_rules! Depcrate_integrations_jiffLocalDateTime {
() => {
// Module: crate::integrations::jiff
// Provides: {"LocalDateTime"}
// Dependencies: {}
# [doc = " Representation of a civil datetime in the Gregorian calendar."] # [doc = ""] # [doc = " Corresponds to a pair of a `LocalDate` and a `LocalTime`. That is, a datetime contains a year,"] # [doc = " month, day, hour, minute, second and the fractional number of nanoseconds."] # [doc = ""] # [doc = " Value is guaranteed to contain a valid date and time. For example, neither `2023-02-29T00:00:00`"] # [doc = " nor `2015-06-30T23:59:60` are valid."] # [doc = ""] # [doc = " [`LocalDateTime` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`jiff::civil::DateTime`][2] for details."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-date-time"] # [doc = " [2]: https://docs.rs/jiff/*/jiff/civil/struct.DateTime.html"] # [graphql_scalar] # [graphql (with = local_date_time , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/local-date-time" ,)] pub type LocalDateTime = jiff :: civil :: DateTime ;
};
}
