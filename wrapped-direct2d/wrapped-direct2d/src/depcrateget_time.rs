// Generated macro for get_time (function)
macro_rules! Depcrateget_time {
() => {
// Module: crate
// Provides: {"get_time"}
// Dependencies: {}
fn get_time (frequency : i64) -> Result < f64 > { unsafe { let mut time = 0 ; QueryPerformanceCounter (& mut time) ? ; Ok (time as f64 / frequency as f64) } }
};
}
