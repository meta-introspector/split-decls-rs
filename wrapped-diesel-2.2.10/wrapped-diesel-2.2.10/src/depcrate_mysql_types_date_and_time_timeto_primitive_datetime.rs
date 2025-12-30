// Generated macro for to_primitive_datetime (function)
macro_rules! Depcrate_mysql_types_date_and_time_timeto_primitive_datetime {
() => {
// Module: crate::mysql::types::date_and_time::time
// Provides: {"to_primitive_datetime"}
// Dependencies: {}
fn to_primitive_datetime (dt : OffsetDateTime) -> PrimitiveDateTime { let dt = dt . to_offset (UtcOffset :: UTC) ; PrimitiveDateTime :: new (dt . date () , dt . time ()) }
};
}
