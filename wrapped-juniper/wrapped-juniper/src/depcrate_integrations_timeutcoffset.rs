// Generated macro for UtcOffset (type)
macro_rules! Depcrate_integrations_timeUtcOffset {
() => {
// Module: crate::integrations::time
// Provides: {"UtcOffset"}
// Dependencies: {}
# [doc = " Offset from UTC in `±hh:mm` format. See [list of database time zones][0]."] # [doc = ""] # [doc = " [`UtcOffset` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`time::UtcOffset`][2] for details."] # [doc = ""] # [doc = " [0]: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones"] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/utc-offset"] # [doc = " [2]: https://docs.rs/time/*/time/struct.UtcOffset.html"] # [graphql_scalar] # [graphql (with = utc_offset , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/utc-offset" ,)] pub type UtcOffset = time :: UtcOffset ;
};
}
