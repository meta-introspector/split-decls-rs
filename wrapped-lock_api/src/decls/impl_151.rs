macro_rules! deps {
    () => {
        RwLock!();
        ArcRwLockWriteGuard!();
        RwLockWriteGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : ? Sized > ArcRwLockWriteGuard < R , T > { # [doc = " Returns a reference to the rwlock, contained in its `Arc`."] pub fn rwlock (s : & Self) -> & Arc < RwLock < R , T > > { & s . rwlock } # [doc = " Unlocks the `RwLock` and returns the `Arc` that was held by the [`ArcRwLockWriteGuard`]."] # [inline] pub fn into_arc (s : Self) -> Arc < RwLock < R , T > > { let s = ManuallyDrop :: new (s) ; unsafe { s . rwlock . raw . unlock_exclusive () ; ptr :: read (& s . rwlock) } } # [doc = " Temporarily unlocks the `RwLock` to execute the given function."] # [doc = ""] # [doc = " This is functionally equivalent to the `unlocked` method on [`RwLockWriteGuard`]."] # [inline] # [track_caller] pub fn unlocked < F , U > (s : & mut Self , f : F) -> U where F : FnOnce () -> U , { unsafe { s . rwlock . raw . unlock_exclusive () ; } defer ! (s . rwlock . raw . lock_exclusive ()) ; f () } }
    };
}

impl_151!();