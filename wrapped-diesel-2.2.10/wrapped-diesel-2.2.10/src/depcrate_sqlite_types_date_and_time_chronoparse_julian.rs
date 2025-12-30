// Generated macro for parse_julian (function)
macro_rules! Depcrate_sqlite_types_date_and_time_chronoparse_julian {
() => {
// Module: crate::sqlite::types::date_and_time::chrono
// Provides: {"parse_julian"}
// Dependencies: {}
fn parse_julian (julian_days : f64) -> Option < NaiveDateTime > { const EPOCH_IN_JULIAN_DAYS : f64 = 2_440_587.5 ; const SECONDS_IN_DAY : f64 = 86400.0 ; let timestamp = (julian_days - EPOCH_IN_JULIAN_DAYS) * SECONDS_IN_DAY ; # [allow (clippy :: cast_possible_truncation)] let seconds = timestamp . trunc () as i64 ; # [allow (clippy :: cast_sign_loss , clippy :: cast_possible_truncation)] let nanos = (timestamp . fract () * 1E9) as u32 ; # [allow (deprecated)] NaiveDateTime :: from_timestamp_opt (seconds , nanos) }
};
}
