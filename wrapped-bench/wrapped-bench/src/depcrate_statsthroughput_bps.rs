// Generated macro for throughput_bps (function)
macro_rules! Depcrate_statsthroughput_bps {
() => {
// Module: crate::stats
// Provides: {"throughput_bps"}
// Dependencies: {}
pub fn throughput_bps (duration : Duration , size : u64) -> f64 { (size as f64) / (duration . as_secs_f64 ()) }
};
}
