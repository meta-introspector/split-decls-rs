// Generated macro for OwnedMutexGuard (struct)
macro_rules! Depcrate_lock_mutexOwnedMutexGuard {
() => {
// Module: crate::lock::mutex
// Provides: {"OwnedMutexGuard"}
// Dependencies: {}
# [doc = " An RAII guard returned by the `lock_owned` and `try_lock_owned` methods."] # [doc = " When this structure is dropped (falls out of scope), the lock will be"] # [doc = " unlocked."] pub struct OwnedMutexGuard < T : ? Sized > { mutex : Arc < Mutex < T > > , }
};
}
