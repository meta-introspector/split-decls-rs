// Generated macro for LocalDateTime (type)
macro_rules! Depcrate_integrations_timeLocalDateTime {
() => {
// Module: crate::integrations::time
// Provides: {"LocalDateTime"}
// Dependencies: {}
# [doc = " Combined date and time (without time zone) in `yyyy-MM-ddTHH:mm:ss` format."] # [doc = ""] # [doc = " [`LocalDateTime` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`time::PrimitiveDateTime`][2] for details."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-date-time"] # [doc = " [2]: https://docs.rs/time/*/time/struct.PrimitiveDateTime.html"] # [graphql_scalar] # [graphql (with = local_date_time , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/local-date-time" ,)] pub type LocalDateTime = time :: PrimitiveDateTime ;
};
}
