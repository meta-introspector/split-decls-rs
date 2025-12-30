// Generated macro for RwLockWriteGuardDetached (struct)
macro_rules! Depcrate_utilRwLockWriteGuardDetached {
() => {
// Module: crate::util
// Provides: {"RwLockWriteGuardDetached"}
// Dependencies: {}
# [doc = " A [`RwLockWriteGuard`], without the data"] pub (crate) struct RwLockWriteGuardDetached < 'a , R : RawRwLock > { lock : & 'a R , _marker : PhantomData < R :: GuardMarker > , }
};
}
