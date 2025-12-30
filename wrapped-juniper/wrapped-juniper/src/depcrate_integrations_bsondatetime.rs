// Generated macro for DateTime (type)
macro_rules! Depcrate_integrations_bsonDateTime {
() => {
// Module: crate::integrations::bson
// Provides: {"DateTime"}
// Dependencies: {}
# [doc = " [BSON date][3] in [RFC 3339][0] format."] # [doc = ""] # [doc = " [BSON datetimes][3] have millisecond precision and are always in UTC (inputs with other"] # [doc = " timezones are coerced)."] # [doc = ""] # [doc = " [`DateTime` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`bson::DateTime`][2] for details."] # [doc = ""] # [doc = " [0]: https://datatracker.ietf.org/doc/html/rfc3339#section-5.6"] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/date-time"] # [doc = " [2]: https://docs.rs/bson/*/bson/struct.DateTime.html"] # [doc = " [3]: https://www.mongodb.com/docs/manual/reference/bson-types#date"] # [graphql_scalar] # [graphql (with = date_time , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/date-time" ,)] type DateTime = bson :: DateTime ;
};
}
