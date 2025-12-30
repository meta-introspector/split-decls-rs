// Generated macro for ZonedDateTime (type)
macro_rules! Depcrate_integrations_jiffZonedDateTime {
() => {
// Module: crate::integrations::jiff
// Provides: {"ZonedDateTime"}
// Dependencies: {}
# [doc = " Time zone aware instant in time."] # [doc = ""] # [doc = " Can be thought of as combination of the following types, all rolled into one:"] # [doc = " - [`Timestamp`][3] for indicating precise instant in time."] # [doc = " - [`DateTime`][4] for indicating \"civil\" calendar date and clock time."] # [doc = " - [`TimeZone`][5] for indicating how to apply time zone transitions while performing arithmetic."] # [doc = ""] # [doc = " [RFC 9557][1] compliant."] # [doc = ""] # [doc = " See also [`jiff::Zoned`][2] for details."] # [doc = ""] # [doc = " [1]: https://datatracker.ietf.org/doc/html/rfc9557#section-4.1"] # [doc = " [2]: https://docs.rs/jiff/latest/jiff/struct.Zoned.html"] # [doc = " [3]: https://docs.rs/jiff/latest/jiff/struct.Timestamp.html"] # [doc = " [4]: https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html"] # [doc = " [5]: https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html"] # [graphql_scalar] # [graphql (with = zoned_date_time , to_output_with = ScalarValue :: from_displayable , parse_token (String) , specified_by_url = "https://datatracker.ietf.org/doc/html/rfc9557#section-4.1" ,)] pub type ZonedDateTime = jiff :: Zoned ;
};
}
