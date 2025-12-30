// Generated macro for duration_to_usecs (function)
macro_rules! Depcrate_pg_types_date_and_time_std_timeduration_to_usecs {
() => {
// Module: crate::pg::types::date_and_time::std_time
// Provides: {"duration_to_usecs"}
// Dependencies: {}
fn duration_to_usecs (duration : Duration) -> u64 { let seconds = duration . as_secs () * USEC_PER_SEC ; let subseconds = duration . subsec_micros () ; seconds + u64 :: from (subseconds) }
};
}
