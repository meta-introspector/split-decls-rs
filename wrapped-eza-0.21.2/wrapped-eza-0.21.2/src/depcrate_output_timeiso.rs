// Generated macro for iso (function)
macro_rules! Depcrate_output_timeiso {
() => {
// Module: crate::output::time
// Provides: {"iso"}
// Dependencies: {}
fn iso (time : & DateTime < FixedOffset >) -> String { if time . year () == * CURRENT_YEAR { time . format ("%m-%d %H:%M") . to_string () } else { time . format ("%Y-%m-%d") . to_string () } }
};
}
