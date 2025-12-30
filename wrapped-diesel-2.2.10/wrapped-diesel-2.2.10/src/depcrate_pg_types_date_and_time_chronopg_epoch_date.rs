// Generated macro for pg_epoch_date (function)
macro_rules! Depcrate_pg_types_date_and_time_chronopg_epoch_date {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"pg_epoch_date"}
// Dependencies: {}
fn pg_epoch_date () -> NaiveDate { NaiveDate :: from_ymd_opt (2000 , 1 , 1) . expect ("This is in supported range of chrono dates") }
};
}
