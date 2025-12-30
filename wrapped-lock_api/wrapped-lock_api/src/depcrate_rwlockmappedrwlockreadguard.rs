// Generated macro for MappedRwLockReadGuard (struct)
macro_rules! Depcrate_rwlockMappedRwLockReadGuard {
() => {
// Module: crate::rwlock
// Provides: {"MappedRwLockReadGuard"}
// Dependencies: {}
# [doc = " An RAII read lock guard returned by `RwLockReadGuard::map`, which can point to a"] # [doc = " subfield of the protected data."] # [doc = ""] # [doc = " The main difference between `MappedRwLockReadGuard` and `RwLockReadGuard` is that the"] # [doc = " former doesn't support temporarily unlocking and re-locking, since that"] # [doc = " could introduce soundness issues if the locked object is modified by another"] # [doc = " thread."] # [clippy :: has_significant_drop] # [must_use = "if unused the RwLock will immediately unlock"] pub struct MappedRwLockReadGuard < 'a , R : RawRwLock , T : ? Sized > { raw : & 'a R , data : * const T , marker : PhantomData < & 'a T > , }
};
}
