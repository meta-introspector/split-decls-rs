// Generated macro for impl_551 (impl)
macro_rules! Depcrate_common_retry_afterimpl_551 {
() => {
// Module: crate::common::retry_after
// Provides: {"impl_551"}
// Dependencies: {}
impl RetryAfter { # [doc = " Create an `RetryAfter` header with a date value."] pub fn date (time : SystemTime) -> RetryAfter { RetryAfter (After :: DateTime (time . into ())) } # [doc = " Create an `RetryAfter` header with a date value."] pub fn delay (dur : Duration) -> RetryAfter { RetryAfter (After :: Delay (dur . into ())) } }
};
}
