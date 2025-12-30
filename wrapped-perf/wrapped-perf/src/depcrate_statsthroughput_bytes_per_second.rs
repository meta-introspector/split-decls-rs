// Generated macro for throughput_bytes_per_second (function)
macro_rules! Depcrate_statsthroughput_bytes_per_second {
() => {
// Module: crate::stats
// Provides: {"throughput_bytes_per_second"}
// Dependencies: {}
fn throughput_bytes_per_second (duration_in_micros : u64 , size : u64) -> f64 { (size as f64) / (duration_in_micros as f64 / 1000000.0) }
};
}
