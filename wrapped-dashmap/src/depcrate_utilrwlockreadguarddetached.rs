// Generated macro for RwLockReadGuardDetached (struct)
macro_rules! Depcrate_utilRwLockReadGuardDetached {
() => {
// Module: crate::util
// Provides: {"RwLockReadGuardDetached"}
// Dependencies: {}
# [doc = " A [`RwLockReadGuard`], without the data"] pub (crate) struct RwLockReadGuardDetached < 'a , R : RawRwLock > { lock : & 'a R , _marker : PhantomData < R :: GuardMarker > , }
};
}
