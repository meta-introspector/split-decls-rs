// Generated macro for relative (function)
macro_rules! Depcrate_output_timerelative {
() => {
// Module: crate::output::time
// Provides: {"relative"}
// Dependencies: {}
fn relative (time : & DateTime < FixedOffset >) -> String { timeago :: Formatter :: new () . ago ("") . convert (Duration :: from_secs (max (0 , Local :: now () . timestamp () - time . timestamp ()) . try_into () . unwrap () ,)) }
};
}
