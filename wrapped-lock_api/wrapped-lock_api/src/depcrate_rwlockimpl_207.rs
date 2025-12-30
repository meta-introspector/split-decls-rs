// Generated macro for impl_207 (impl)
macro_rules! Depcrate_rwlockimpl_207 {
() => {
// Module: crate::rwlock
// Provides: {"impl_207"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgradeTimed , T : ? Sized > ArcRwLockUpgradableReadGuard < R , T > { # [doc = " Tries to atomically upgrade an upgradable read lock into an exclusive"] # [doc = " write lock, until a timeout is reached."] # [doc = ""] # [doc = " If the access could not be granted before the timeout expires, then"] # [doc = " the current guard is returned."] # [track_caller] pub fn try_upgrade_for (s : Self , timeout : R :: Duration ,) -> Result < ArcRwLockWriteGuard < R , T > , Self > { if unsafe { s . rwlock . raw . try_upgrade_for (timeout) } { let s = ManuallyDrop :: new (s) ; let rwlock = unsafe { ptr :: read (& s . rwlock) } ; Ok (ArcRwLockWriteGuard { rwlock , marker : PhantomData , }) } else { Err (s) } } # [doc = " Tries to atomically upgrade an upgradable read lock into an exclusive"] # [doc = " write lock, until a timeout is reached."] # [doc = ""] # [doc = " If the access could not be granted before the timeout expires, then"] # [doc = " the current guard is returned."] # [inline] # [track_caller] pub fn try_upgrade_until (s : Self , timeout : R :: Instant ,) -> Result < ArcRwLockWriteGuard < R , T > , Self > { if unsafe { s . rwlock . raw . try_upgrade_until (timeout) } { let s = ManuallyDrop :: new (s) ; let rwlock = unsafe { ptr :: read (& s . rwlock) } ; Ok (ArcRwLockWriteGuard { rwlock , marker : PhantomData , }) } else { Err (s) } } }
};
}
