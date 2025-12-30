// Generated macro for Mutex (struct)
macro_rules! Depcrate_concurrency_syncMutex {
() => {
// Module: crate::concurrency::sync
// Provides: {"Mutex"}
// Dependencies: {}
# [doc = " The mutex state."] # [derive (Default , Debug)] struct Mutex { # [doc = " The thread that currently owns the lock."] owner : Option < ThreadId > , # [doc = " How many times the mutex was locked by the owner."] lock_count : usize , # [doc = " The queue of threads waiting for this mutex."] queue : VecDeque < ThreadId > , # [doc = " Mutex clock. This tracks the moment of the last unlock."] clock : VClock , }
};
}
