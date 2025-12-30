// Generated macro for LocalDateTime (type)
macro_rules! Depcrate_integrations_chronoLocalDateTime {
() => {
// Module: crate::integrations::chrono
// Provides: {"LocalDateTime"}
// Dependencies: {}
# [doc = " Combined date and time (without time zone) in `yyyy-MM-ddTHH:mm:ss` format."] # [doc = ""] # [doc = " [`LocalDateTime` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`chrono::NaiveDateTime`][2] for details."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-date-time"] # [doc = " [2]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html"] # [graphql_scalar] # [graphql (with = local_date_time , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/local-date-time" ,)] pub type LocalDateTime = chrono :: NaiveDateTime ;
};
}
