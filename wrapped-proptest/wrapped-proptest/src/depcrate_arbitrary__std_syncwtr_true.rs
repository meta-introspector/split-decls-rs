// Generated macro for wtr_true (function)
macro_rules! Depcrate_arbitrary__std_syncwtr_true {
() => {
// Module: crate::arbitrary::_std::sync
// Provides: {"wtr_true"}
// Dependencies: {}
fn wtr_true () -> WaitTimeoutResult { let cvar = Condvar :: new () ; let lock = Mutex :: new (()) ; let wt = cvar . wait_timeout (lock . lock () . unwrap () , Duration :: from_millis (0)) ; let (_unused , wtr) = wt . unwrap () ; wtr }
};
}
