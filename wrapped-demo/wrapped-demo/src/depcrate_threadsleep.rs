// Generated macro for sleep (function)
macro_rules! Depcrate_threadsleep {
() => {
// Module: crate::thread
// Provides: {"sleep"}
// Dependencies: {}
pub fn sleep () { eprintln ! () ; let duration = Duration :: from_millis (100) ; let now = Instant :: now () ; thread :: sleep (duration) ; let elapsed = now . elapsed () ; eprintln ! ("Measured time for {duration:?} sleep: {elapsed:?}") ; assert ! (elapsed >= duration) ; let expected_delay = if cfg ! (debug_assertions) { Duration :: from_secs (1) } else { Duration :: from_millis (5) } ; assert ! (elapsed <= duration + expected_delay) ; }
};
}
