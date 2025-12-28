macro_rules! deps {
    () => {
        RawRwLockFair!();
        RwLockWriteGuard!();
        ArcRwLockWriteGuard!();
        RwLock!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLockFair , T : ? Sized > ArcRwLockWriteGuard < R , T > { # [doc = " Unlocks the `RwLock` using a fair unlock protocol."] # [doc = ""] # [doc = " This is functionally equivalent to the `unlock_fair` method on [`RwLockWriteGuard`]."] # [inline] # [track_caller] pub fn unlock_fair (s : Self) { drop (Self :: into_arc_fair (s)) ; } # [doc = " Unlocks the `RwLock` using a fair unlock protocol and returns the `Arc` that was held by the [`ArcRwLockWriteGuard`]."] # [inline] pub fn into_arc_fair (s : Self) -> Arc < RwLock < R , T > > { let s = ManuallyDrop :: new (s) ; unsafe { s . rwlock . raw . unlock_exclusive_fair () ; ptr :: read (& s . rwlock) } } # [doc = " Temporarily unlocks the `RwLock` to execute the given function."] # [doc = ""] # [doc = " This is functionally equivalent to the `unlocked_fair` method on [`RwLockWriteGuard`]."] # [inline] # [track_caller] pub fn unlocked_fair < F , U > (s : & mut Self , f : F) -> U where F : FnOnce () -> U , { unsafe { s . rwlock . raw . unlock_exclusive_fair () ; } defer ! (s . rwlock . raw . lock_exclusive ()) ; f () } # [doc = " Temporarily yields the `RwLock` to a waiting thread if there is one."] # [doc = ""] # [doc = " This method is functionally equivalent to the `bump` method on [`RwLockWriteGuard`]."] # [inline] # [track_caller] pub fn bump (s : & mut Self) { unsafe { s . rwlock . raw . bump_exclusive () ; } } }
    };
}

impl_154!()