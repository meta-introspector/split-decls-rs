// Generated macro for MappedMutexGuard (struct)
macro_rules! Depcrate_lock_mutexMappedMutexGuard {
() => {
// Module: crate::lock::mutex
// Provides: {"MappedMutexGuard"}
// Dependencies: {}
# [doc = " An RAII guard returned by the `MutexGuard::map` and `MappedMutexGuard::map` methods."] # [doc = " When this structure is dropped (falls out of scope), the lock will be unlocked."] pub struct MappedMutexGuard < 'a , T : ? Sized , U : ? Sized > { mutex : & 'a Mutex < T > , value : * mut U , _marker : PhantomData < & 'a mut U > , }
};
}
