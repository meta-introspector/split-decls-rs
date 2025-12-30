// Generated macro for ControllerMetrics (struct)
macro_rules! Depcrate_congestionControllerMetrics {
() => {
// Module: crate::congestion
// Provides: {"ControllerMetrics"}
// Dependencies: {}
# [doc = " Common congestion controller metrics"] # [derive (Default)] # [non_exhaustive] pub struct ControllerMetrics { # [doc = " Congestion window (bytes)"] pub congestion_window : u64 , # [doc = " Slow start threshold (bytes)"] pub ssthresh : Option < u64 > , # [doc = " Pacing rate (bits/s)"] pub pacing_rate : Option < u64 > , }
};
}
