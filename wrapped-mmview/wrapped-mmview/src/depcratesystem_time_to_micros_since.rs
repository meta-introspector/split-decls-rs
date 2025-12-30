// Generated macro for system_time_to_micros_since (function)
macro_rules! Depcratesystem_time_to_micros_since {
() => {
// Module: crate
// Provides: {"system_time_to_micros_since"}
// Dependencies: {}
fn system_time_to_micros_since (t : SystemTime , since : SystemTime) -> u128 { t . duration_since (since) . unwrap_or (Duration :: from_nanos (0)) . as_micros () }
};
}
