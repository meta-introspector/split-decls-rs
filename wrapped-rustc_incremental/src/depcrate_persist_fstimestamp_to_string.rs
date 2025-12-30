// Generated macro for timestamp_to_string (function)
macro_rules! Depcrate_persist_fstimestamp_to_string {
() => {
// Module: crate::persist::fs
// Provides: {"timestamp_to_string"}
// Dependencies: {}
fn timestamp_to_string (timestamp : SystemTime) -> BaseNString { let duration = timestamp . duration_since (UNIX_EPOCH) . unwrap () ; let micros : u64 = duration . as_micros () . try_into () . unwrap () ; micros . to_base_fixed_len (CASE_INSENSITIVE) }
};
}
