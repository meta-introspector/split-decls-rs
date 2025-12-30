// Generated macro for pg_epoch (function)
macro_rules! Depcrate_pg_types_date_and_time_chronopg_epoch {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"pg_epoch"}
// Dependencies: {}
fn pg_epoch () -> NaiveDateTime { NaiveDate :: from_ymd_opt (2000 , 1 , 1) . expect ("This is in supported range of chrono dates") . and_hms_opt (0 , 0 , 0) . expect ("This is a valid input") }
};
}
