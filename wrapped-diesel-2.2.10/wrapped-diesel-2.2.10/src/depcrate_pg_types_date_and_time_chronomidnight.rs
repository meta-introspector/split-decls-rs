// Generated macro for midnight (function)
macro_rules! Depcrate_pg_types_date_and_time_chronomidnight {
() => {
// Module: crate::pg::types::date_and_time::chrono
// Provides: {"midnight"}
// Dependencies: {}
fn midnight () -> NaiveTime { NaiveTime :: from_hms_opt (0 , 0 , 0) . expect ("This is a valid hms spec") }
};
}
