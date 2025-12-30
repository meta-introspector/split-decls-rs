// Generated macro for impl_196 (impl)
macro_rules! Depcrate_rwlockimpl_196 {
() => {
// Module: crate::rwlock
// Provides: {"impl_196"}
// Dependencies: {}
impl < 'a , R : RawRwLockUpgradeTimed + 'a , T : ? Sized + 'a > RwLockUpgradableReadGuard < 'a , R , T > { # [doc = " Tries to atomically upgrade an upgradable read lock into an exclusive"] # [doc = " write lock, until a timeout is reached."] # [doc = ""] # [doc = " If the access could not be granted before the timeout expires, then"] # [doc = " the current guard is returned."] # [track_caller] pub fn try_upgrade_for (s : Self , timeout : R :: Duration ,) -> Result < RwLockWriteGuard < 'a , R , T > , Self > { if unsafe { s . rwlock . raw . try_upgrade_for (timeout) } { let rwlock = s . rwlock ; mem :: forget (s) ; Ok (RwLockWriteGuard { rwlock , marker : PhantomData , }) } else { Err (s) } } # [doc = " Tries to atomically upgrade an upgradable read lock into an exclusive"] # [doc = " write lock, until a timeout is reached."] # [doc = ""] # [doc = " If the access could not be granted before the timeout expires, then"] # [doc = " the current guard is returned."] # [inline] # [track_caller] pub fn try_upgrade_until (s : Self , timeout : R :: Instant ,) -> Result < RwLockWriteGuard < 'a , R , T > , Self > { if unsafe { s . rwlock . raw . try_upgrade_until (timeout) } { let rwlock = s . rwlock ; mem :: forget (s) ; Ok (RwLockWriteGuard { rwlock , marker : PhantomData , }) } else { Err (s) } } }
};
}
