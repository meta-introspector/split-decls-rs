// Generated macro for align_time (function)
macro_rules! Depcratealign_time {
() => {
// Module: crate
// Provides: {"align_time"}
// Dependencies: {}
fn align_time () { fn sample () -> u64 { time :: SystemTime :: now () . duration_since (time :: SystemTime :: UNIX_EPOCH) . unwrap () . as_secs () } let start_secs = sample () ; while start_secs == sample () { thread :: sleep (time :: Duration :: from_millis (20)) ; } }
};
}
