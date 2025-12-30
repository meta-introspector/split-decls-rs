// Generated macro for OwnedMutexLockFuture (struct)
macro_rules! Depcrate_lock_mutexOwnedMutexLockFuture {
() => {
// Module: crate::lock::mutex
// Provides: {"OwnedMutexLockFuture"}
// Dependencies: {}
# [doc = " A future which resolves when the target mutex has been successfully acquired, owned version."] pub struct OwnedMutexLockFuture < T : ? Sized > { mutex : Option < Arc < Mutex < T > > > , wait_key : usize , }
};
}
