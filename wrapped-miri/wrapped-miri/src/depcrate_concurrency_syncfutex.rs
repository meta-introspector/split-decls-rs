// Generated macro for Futex (struct)
macro_rules! Depcrate_concurrency_syncFutex {
() => {
// Module: crate::concurrency::sync
// Provides: {"Futex"}
// Dependencies: {}
# [doc = " The futex state."] # [derive (Default , Debug)] struct Futex { waiters : Vec < FutexWaiter > , # [doc = " Tracks the happens-before relationship"] # [doc = " between a futex-wake and a futex-wait"] # [doc = " during a non-spurious wake event."] # [doc = " Contains the clock of the last thread to"] # [doc = " perform a futex-wake."] clock : VClock , }
};
}
