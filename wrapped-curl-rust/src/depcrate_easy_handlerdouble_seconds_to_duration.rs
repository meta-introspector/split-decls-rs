// Generated macro for double_seconds_to_duration (function)
macro_rules! Depcrate_easy_handlerdouble_seconds_to_duration {
() => {
// Module: crate::easy::handler
// Provides: {"double_seconds_to_duration"}
// Dependencies: {}
fn double_seconds_to_duration (seconds : f64) -> Duration { let whole_seconds = seconds . trunc () as u64 ; let nanos = seconds . fract () * 1_000_000_000f64 ; Duration :: new (whole_seconds , nanos as u32) }
};
}
