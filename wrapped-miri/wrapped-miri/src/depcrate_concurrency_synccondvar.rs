// Generated macro for Condvar (struct)
macro_rules! Depcrate_concurrency_syncCondvar {
() => {
// Module: crate::concurrency::sync
// Provides: {"Condvar"}
// Dependencies: {}
# [doc = " The conditional variable state."] # [derive (Default , Debug)] struct Condvar { waiters : VecDeque < ThreadId > , # [doc = " Tracks the happens-before relationship"] # [doc = " between a cond-var signal and a cond-var"] # [doc = " wait during a non-spurious signal event."] # [doc = " Contains the clock of the last thread to"] # [doc = " perform a condvar-signal."] clock : VClock , }
};
}
