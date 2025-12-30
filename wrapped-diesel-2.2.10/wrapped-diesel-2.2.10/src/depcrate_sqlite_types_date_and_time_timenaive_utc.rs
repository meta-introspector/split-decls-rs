// Generated macro for naive_utc (function)
macro_rules! Depcrate_sqlite_types_date_and_time_timenaive_utc {
() => {
// Module: crate::sqlite::types::date_and_time::time
// Provides: {"naive_utc"}
// Dependencies: {}
fn naive_utc (dt : OffsetDateTime) -> PrimitiveDateTime { let dt = dt . to_offset (UtcOffset :: UTC) ; PrimitiveDateTime :: new (dt . date () , dt . time ()) }
};
}
