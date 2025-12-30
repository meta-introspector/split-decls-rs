// Generated macro for usecs_to_duration (function)
macro_rules! Depcrate_pg_types_date_and_time_std_timeusecs_to_duration {
() => {
// Module: crate::pg::types::date_and_time::std_time
// Provides: {"usecs_to_duration"}
// Dependencies: {}
fn usecs_to_duration (usecs_passed : u64) -> Duration { let seconds = usecs_passed / USEC_PER_SEC ; let subsecond_usecs = usecs_passed % USEC_PER_SEC ; let subseconds = subsecond_usecs as u32 * NANO_PER_USEC ; Duration :: new (seconds , subseconds) }
};
}
