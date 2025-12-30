// Generated macro for parse_julian (function)
macro_rules! Depcrate_sqlite_types_date_and_time_timeparse_julian {
() => {
// Module: crate::sqlite::types::date_and_time::time
// Provides: {"parse_julian"}
// Dependencies: {}
fn parse_julian (julian_days : f64) -> Result < PrimitiveDateTime , ComponentRange > { const EPOCH_IN_JULIAN_DAYS : f64 = 2_440_587.5 ; const SECONDS_IN_DAY : f64 = 86400.0 ; let timestamp = (julian_days - EPOCH_IN_JULIAN_DAYS) * SECONDS_IN_DAY ; # [allow (clippy :: cast_possible_truncation)] OffsetDateTime :: from_unix_timestamp_nanos ((timestamp * 1E9) as i128) . map (naive_utc) }
};
}
