// Generated macro for perf_to_system (function)
macro_rules! Depcrateperf_to_system {
() => {
// Module: crate
// Provides: {"perf_to_system"}
// Dependencies: {}
fn perf_to_system (amt : f64) -> SystemTime { let secs = (amt as u64) / 1_000 ; let nanos = (((amt as u64) % 1_000) as u32) * 1_000_000 ; UNIX_EPOCH + Duration :: new (secs , nanos) }
};
}
