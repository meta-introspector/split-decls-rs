// Generated macro for long (function)
macro_rules! Depcrate_output_timelong {
() => {
// Module: crate::output::time
// Provides: {"long"}
// Dependencies: {}
fn long (time : & DateTime < FixedOffset >) -> String { time . format ("%Y-%m-%d %H:%M") . to_string () }
};
}
