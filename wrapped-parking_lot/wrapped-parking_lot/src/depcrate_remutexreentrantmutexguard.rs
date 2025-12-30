// Generated macro for ReentrantMutexGuard (type)
macro_rules! Depcrate_remutexReentrantMutexGuard {
() => {
// Module: crate::remutex
// Provides: {"ReentrantMutexGuard"}
// Dependencies: {}
# [doc = " An RAII implementation of a \"scoped lock\" of a reentrant mutex. When this structure"] # [doc = " is dropped (falls out of scope), the lock will be unlocked."] # [doc = ""] # [doc = " The data protected by the mutex can be accessed through this guard via its"] # [doc = " `Deref` implementation."] pub type ReentrantMutexGuard < 'a , T > = lock_api :: ReentrantMutexGuard < 'a , RawMutex , RawThreadId , T > ;
};
}
