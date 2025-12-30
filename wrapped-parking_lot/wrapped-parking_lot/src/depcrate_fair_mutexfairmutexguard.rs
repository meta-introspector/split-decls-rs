// Generated macro for FairMutexGuard (type)
macro_rules! Depcrate_fair_mutexFairMutexGuard {
() => {
// Module: crate::fair_mutex
// Provides: {"FairMutexGuard"}
// Dependencies: {}
# [doc = " An RAII implementation of a \"scoped lock\" of a mutex. When this structure is"] # [doc = " dropped (falls out of scope), the lock will be unlocked."] # [doc = ""] # [doc = " The data protected by the mutex can be accessed through this guard via its"] # [doc = " `Deref` and `DerefMut` implementations."] pub type FairMutexGuard < 'a , T > = lock_api :: MutexGuard < 'a , RawFairMutex , T > ;
};
}
