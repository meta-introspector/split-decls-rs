// Generated macro for impl_193 (impl)
macro_rules! Depcrate_rwlockimpl_193 {
() => {
// Module: crate::rwlock
// Provides: {"impl_193"}
// Dependencies: {}
impl < 'a , R : RawRwLockUpgrade + 'a , T : ? Sized + 'a > RwLockUpgradableReadGuard < 'a , R , T > { # [doc = " Returns a reference to the original reader-writer lock object."] pub fn rwlock (s : & Self) -> & 'a RwLock < R , T > { s . rwlock } # [doc = " Temporarily unlocks the `RwLock` to execute the given function."] # [doc = ""] # [doc = " This is safe because `&mut` guarantees that there exist no other"] # [doc = " references to the data protected by the `RwLock`."] # [inline] # [track_caller] pub fn unlocked < F , U > (s : & mut Self , f : F) -> U where F : FnOnce () -> U , { unsafe { s . rwlock . raw . unlock_upgradable () ; } defer ! (s . rwlock . raw . lock_upgradable ()) ; f () } # [doc = " Atomically upgrades an upgradable read lock lock into an exclusive write lock,"] # [doc = " blocking the current thread until it can be acquired."] # [track_caller] pub fn upgrade (s : Self) -> RwLockWriteGuard < 'a , R , T > { unsafe { s . rwlock . raw . upgrade () ; } let rwlock = s . rwlock ; mem :: forget (s) ; RwLockWriteGuard { rwlock , marker : PhantomData , } } # [doc = " Tries to atomically upgrade an upgradable read lock into an exclusive write lock."] # [doc = ""] # [doc = " If the access could not be granted at this time, then the current guard is returned."] # [track_caller] pub fn try_upgrade (s : Self) -> Result < RwLockWriteGuard < 'a , R , T > , Self > { if unsafe { s . rwlock . raw . try_upgrade () } { let rwlock = s . rwlock ; mem :: forget (s) ; Ok (RwLockWriteGuard { rwlock , marker : PhantomData , }) } else { Err (s) } } }
};
}
