// Generated macro for MutexGuard (struct)
macro_rules! Depcrate_mutexMutexGuard {
() => {
// Module: crate::mutex
// Provides: {"MutexGuard"}
// Dependencies: {}
# [doc = " A guard that releases the mutex when dropped."] # [clippy :: has_significant_drop] pub struct MutexGuard < 'a , T : ? Sized > (& 'a Mutex < T >) ;
};
}
