// Generated macro for MappedMutexGuard (struct)
macro_rules! Depcrate_mutexMappedMutexGuard {
() => {
// Module: crate::mutex
// Provides: {"MappedMutexGuard"}
// Dependencies: {}
# [doc = " An RAII mutex guard returned by `MutexGuard::map`, which can point to a"] # [doc = " subfield of the protected data."] # [doc = ""] # [doc = " The main difference between `MappedMutexGuard` and `MutexGuard` is that the"] # [doc = " former doesn't support temporarily unlocking and re-locking, since that"] # [doc = " could introduce soundness issues if the locked object is modified by another"] # [doc = " thread."] # [clippy :: has_significant_drop] # [must_use = "if unused the Mutex will immediately unlock"] pub struct MappedMutexGuard < 'a , R : RawMutex , T : ? Sized > { raw : & 'a R , data : * mut T , marker : PhantomData < & 'a mut T > , }
};
}
