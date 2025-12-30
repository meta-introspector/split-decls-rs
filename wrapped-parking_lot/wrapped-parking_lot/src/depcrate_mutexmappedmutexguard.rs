// Generated macro for MappedMutexGuard (type)
macro_rules! Depcrate_mutexMappedMutexGuard {
() => {
// Module: crate::mutex
// Provides: {"MappedMutexGuard"}
// Dependencies: {}
# [doc = " An RAII mutex guard returned by `MutexGuard::map`, which can point to a"] # [doc = " subfield of the protected data."] # [doc = ""] # [doc = " The main difference between `MappedMutexGuard` and `MutexGuard` is that the"] # [doc = " former doesn't support temporarily unlocking and re-locking, since that"] # [doc = " could introduce soundness issues if the locked object is modified by another"] # [doc = " thread."] pub type MappedMutexGuard < 'a , T > = lock_api :: MappedMutexGuard < 'a , RawMutex , T > ;
};
}
