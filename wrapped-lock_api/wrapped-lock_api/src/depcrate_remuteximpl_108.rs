// Generated macro for impl_108 (impl)
macro_rules! Depcrate_remuteximpl_108 {
() => {
// Module: crate::remutex
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'a , R : RawMutexFair + 'a , G : GetThreadId + 'a , T : ? Sized + 'a > MappedReentrantMutexGuard < 'a , R , G , T > { # [doc = " Unlocks the mutex using a fair unlock protocol."] # [doc = ""] # [doc = " By default, mutexes are unfair and allow the current thread to re-lock"] # [doc = " the mutex before another has the chance to acquire the lock, even if"] # [doc = " that thread has been blocked on the mutex for a long time. This is the"] # [doc = " default because it allows much higher throughput as it avoids forcing a"] # [doc = " context switch on every mutex unlock. This can result in one thread"] # [doc = " acquiring a mutex many more times than other threads."] # [doc = ""] # [doc = " However in some cases it can be beneficial to ensure fairness by forcing"] # [doc = " the lock to pass on to a waiting thread if there is one. This is done by"] # [doc = " using this method instead of dropping the `ReentrantMutexGuard` normally."] # [inline] # [track_caller] pub fn unlock_fair (s : Self) { unsafe { s . raw . unlock_fair () ; } mem :: forget (s) ; } }
};
}
