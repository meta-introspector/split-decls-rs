// Generated macro for wtr_false (function)
macro_rules! Depcrate_arbitrary__std_syncwtr_false {
() => {
// Module: crate::arbitrary::_std::sync
// Provides: {"wtr_false"}
// Dependencies: {}
fn wtr_false () -> WaitTimeoutResult { let cvar = Arc :: new (Condvar :: new ()) ; let cvar2 = cvar . clone () ; thread :: spawn (move | | { cvar2 . notify_one () ; }) ; let lock = Mutex :: new (()) ; let wt = cvar . wait_timeout (lock . lock () . unwrap () , Duration :: from_millis (1)) ; let (_unused , wtr) = wt . unwrap () ; wtr }
};
}
