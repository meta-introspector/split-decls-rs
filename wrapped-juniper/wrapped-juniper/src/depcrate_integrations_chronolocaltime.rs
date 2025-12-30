// Generated macro for LocalTime (type)
macro_rules! Depcrate_integrations_chronoLocalTime {
() => {
// Module: crate::integrations::chrono
// Provides: {"LocalTime"}
// Dependencies: {}
# [doc = " Clock time within a given date (without time zone) in `HH:mm[:ss[.SSS]]`"] # [doc = " format."] # [doc = ""] # [doc = " All minutes are assumed to have exactly 60 seconds; no attempt is made to"] # [doc = " handle leap seconds (either positive or negative)."] # [doc = ""] # [doc = " [`LocalTime` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`chrono::NaiveTime`][2] for details."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/local-time"] # [doc = " [2]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html"] # [graphql_scalar] # [graphql (with = local_time , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/local-time" ,)] pub type LocalTime = chrono :: NaiveTime ;
};
}
