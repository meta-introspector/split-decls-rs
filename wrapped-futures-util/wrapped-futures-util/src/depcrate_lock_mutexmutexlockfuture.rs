// Generated macro for MutexLockFuture (struct)
macro_rules! Depcrate_lock_mutexMutexLockFuture {
() => {
// Module: crate::lock::mutex
// Provides: {"MutexLockFuture"}
// Dependencies: {}
# [doc = " A future which resolves when the target mutex has been successfully acquired."] pub struct MutexLockFuture < 'a , T : ? Sized > { mutex : Option < & 'a Mutex < T > > , wait_key : usize , }
};
}
