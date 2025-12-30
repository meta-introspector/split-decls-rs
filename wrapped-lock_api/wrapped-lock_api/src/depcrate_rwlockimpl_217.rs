// Generated macro for impl_217 (impl)
macro_rules! Depcrate_rwlockimpl_217 {
() => {
// Module: crate::rwlock
// Provides: {"impl_217"}
// Dependencies: {}
impl < 'a , R : RawRwLockFair + 'a , T : ? Sized + 'a > MappedRwLockReadGuard < 'a , R , T > { # [doc = " Unlocks the `RwLock` using a fair unlock protocol."] # [doc = ""] # [doc = " By default, `RwLock` is unfair and allow the current thread to re-lock"] # [doc = " the `RwLock` before another has the chance to acquire the lock, even if"] # [doc = " that thread has been blocked on the `RwLock` for a long time. This is"] # [doc = " the default because it allows much higher throughput as it avoids"] # [doc = " forcing a context switch on every `RwLock` unlock. This can result in one"] # [doc = " thread acquiring a `RwLock` many more times than other threads."] # [doc = ""] # [doc = " However in some cases it can be beneficial to ensure fairness by forcing"] # [doc = " the lock to pass on to a waiting thread if there is one. This is done by"] # [doc = " using this method instead of dropping the `MappedRwLockReadGuard` normally."] # [inline] # [track_caller] pub fn unlock_fair (s : Self) { unsafe { s . raw . unlock_shared_fair () ; } mem :: forget (s) ; } }
};
}
