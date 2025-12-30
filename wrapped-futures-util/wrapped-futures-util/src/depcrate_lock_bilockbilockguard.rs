// Generated macro for BiLockGuard (struct)
macro_rules! Depcrate_lock_bilockBiLockGuard {
() => {
// Module: crate::lock::bilock
// Provides: {"BiLockGuard"}
// Dependencies: {}
# [doc = " Returned RAII guard from the `poll_lock` method."] # [doc = ""] # [doc = " This structure acts as a sentinel to the data in the `BiLock<T>` itself,"] # [doc = " implementing `Deref` and `DerefMut` to `T`. When dropped, the lock will be"] # [doc = " unlocked."] # [derive (Debug)] # [cfg_attr (docsrs , doc (cfg (feature = "bilock")))] pub struct BiLockGuard < 'a , T > { bilock : & 'a BiLock < T > , }
};
}
