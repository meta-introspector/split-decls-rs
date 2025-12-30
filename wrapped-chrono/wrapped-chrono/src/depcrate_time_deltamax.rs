// Generated macro for MAX (const)
macro_rules! Depcrate_time_deltaMAX {
() => {
// Module: crate::time_delta
// Provides: {"MAX"}
// Dependencies: {}
# [doc = " The maximum possible `TimeDelta`: `i64::MAX` milliseconds."] pub (crate) const MAX : TimeDelta = TimeDelta { secs : i64 :: MAX / MILLIS_PER_SEC , nanos : (i64 :: MAX % MILLIS_PER_SEC) as i32 * NANOS_PER_MILLI , } ;
};
}
