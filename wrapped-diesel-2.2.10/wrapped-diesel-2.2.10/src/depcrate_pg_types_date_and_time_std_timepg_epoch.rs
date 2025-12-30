// Generated macro for pg_epoch (function)
macro_rules! Depcrate_pg_types_date_and_time_std_timepg_epoch {
() => {
// Module: crate::pg::types::date_and_time::std_time
// Provides: {"pg_epoch"}
// Dependencies: {}
fn pg_epoch () -> SystemTime { let thirty_years = Duration :: from_secs (946_684_800) ; UNIX_EPOCH + thirty_years }
};
}
