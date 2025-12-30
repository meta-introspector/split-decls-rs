// Generated macro for full (function)
macro_rules! Depcrate_output_timefull {
() => {
// Module: crate::output::time
// Provides: {"full"}
// Dependencies: {}
fn full (time : & DateTime < FixedOffset >) -> String { time . format ("%Y-%m-%d %H:%M:%S.%f %z") . to_string () }
};
}
