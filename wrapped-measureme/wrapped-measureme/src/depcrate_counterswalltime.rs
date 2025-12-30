// Generated macro for WallTime (struct)
macro_rules! Depcrate_countersWallTime {
() => {
// Module: crate::counters
// Provides: {"WallTime"}
// Dependencies: {}
# [doc = " \"Monotonic clock\" with nanosecond precision (using [`std::time::Instant`])."] # [doc = ""] # [doc = " Can be obtained with `Counter::by_name(\"wall-time\")`."] pub struct WallTime { start : Instant , }
};
}
