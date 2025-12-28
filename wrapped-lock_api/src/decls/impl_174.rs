macro_rules! deps {
    () => {
        ArcRwLockUpgradableReadGuard!();
        RwLockUpgradableReadGuard!();
        RawRwLockUpgradeFair!();
        RwLock!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgradeFair , T : ? Sized > ArcRwLockUpgradableReadGuard < R , T > { # [doc = " Unlocks the `RwLock` using a fair unlock protocol."] # [doc = ""] # [doc = " This is functionally identical to the `unlock_fair` method on [`RwLockUpgradableReadGuard`]."] # [inline] # [track_caller] pub fn unlock_fair (s : Self) { drop (Self :: into_arc_fair (s)) ; } # [doc = " Unlocks the `RwLock` using a fair unlock protocol and returns the `Arc` that was held by the [`ArcRwLockUpgradableReadGuard`]."] # [inline] pub fn into_arc_fair (s : Self) -> Arc < RwLock < R , T > > { let s = ManuallyDrop :: new (s) ; unsafe { s . rwlock . raw . unlock_upgradable_fair () ; ptr :: read (& s . rwlock) } } # [doc = " Temporarily unlocks the `RwLock` to execute the given function."] # [doc = ""] # [doc = " This is functionally equivalent to the `unlocked_fair` method on [`RwLockUpgradableReadGuard`]."] # [inline] # [track_caller] pub fn unlocked_fair < F , U > (s : & mut Self , f : F) -> U where F : FnOnce () -> U , { unsafe { s . rwlock . raw . unlock_upgradable_fair () ; } defer ! (s . rwlock . raw . lock_upgradable ()) ; f () } # [doc = " Temporarily yields the `RwLock` to a waiting thread if there is one."] # [doc = ""] # [doc = " This method is functionally equivalent to calling `bump` on [`RwLockUpgradableReadGuard`]."] # [inline] # [track_caller] pub fn bump (s : & mut Self) { unsafe { s . rwlock . raw . bump_upgradable () ; } } }
    };
}

impl_174!();