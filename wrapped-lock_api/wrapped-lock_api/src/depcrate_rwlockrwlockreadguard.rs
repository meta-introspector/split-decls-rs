// Generated macro for RwLockReadGuard (struct)
macro_rules! Depcrate_rwlockRwLockReadGuard {
() => {
// Module: crate::rwlock
// Provides: {"RwLockReadGuard"}
// Dependencies: {}
# [doc = " RAII structure used to release the shared read access of a lock when"] # [doc = " dropped."] # [clippy :: has_significant_drop] # [must_use = "if unused the RwLock will immediately unlock"] pub struct RwLockReadGuard < 'a , R : RawRwLock , T : ? Sized > { rwlock : & 'a RwLock < R , T > , marker : PhantomData < (& 'a T , R :: GuardMarker) > , }
};
}
