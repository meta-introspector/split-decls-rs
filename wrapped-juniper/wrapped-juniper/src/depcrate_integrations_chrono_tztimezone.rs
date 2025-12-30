// Generated macro for TimeZone (type)
macro_rules! Depcrate_integrations_chrono_tzTimeZone {
() => {
// Module: crate::integrations::chrono_tz
// Provides: {"TimeZone"}
// Dependencies: {}
# [doc = " Timezone based on [`IANA` database][0]."] # [doc = ""] # [doc = " See [\"List of tz database time zones\"][3] `TZ database name` column for"] # [doc = " available names."] # [doc = ""] # [doc = " [`TimeZone` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`chrono_tz::Tz`][2] for details."] # [doc = ""] # [doc = " [0]: https://www.iana.org/time-zones"] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/time-zone"] # [doc = " [2]: https://docs.rs/chrono-tz/*/chrono_tz/enum.Tz.html"] # [doc = " [3]: https://en.wikipedia.org/wiki/List_of_tz_database_time_zones"] # [graphql_scalar] # [graphql (with = tz , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/time-zone" ,)] pub type TimeZone = chrono_tz :: Tz ;
};
}
