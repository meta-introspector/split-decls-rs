// Generated macro for impl_184 (impl)
macro_rules! Depcrate_rwlockimpl_184 {
() => {
// Module: crate::rwlock
// Provides: {"impl_184"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgradeDowngrade , T : ? Sized > ArcRwLockWriteGuard < R , T > { # [doc = " Atomically downgrades a write lock into an upgradable read lock without allowing any"] # [doc = " writers to take exclusive access of the lock in the meantime."] # [doc = ""] # [doc = " This is functionally identical to the `downgrade_to_upgradable` method on [`RwLockWriteGuard`]."] # [track_caller] pub fn downgrade_to_upgradable (s : Self) -> ArcRwLockUpgradableReadGuard < R , T > { unsafe { s . rwlock . raw . downgrade_to_upgradable () ; } let s = ManuallyDrop :: new (s) ; let rwlock = unsafe { ptr :: read (& s . rwlock) } ; ArcRwLockUpgradableReadGuard { rwlock , marker : PhantomData , } } }
};
}
