// Generated macro for FutexWaiter (struct)
macro_rules! Depcrate_concurrency_syncFutexWaiter {
() => {
// Module: crate::concurrency::sync
// Provides: {"FutexWaiter"}
// Dependencies: {}
# [doc = " A thread waiting on a futex."] # [derive (Debug)] struct FutexWaiter { # [doc = " The thread that is waiting on this futex."] thread : ThreadId , # [doc = " The bitset used by FUTEX_*_BITSET, or u32::MAX for other operations."] bitset : u32 , }
};
}
