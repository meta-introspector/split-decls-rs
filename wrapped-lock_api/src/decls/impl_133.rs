macro_rules! deps {
    () => {
        RawRwLockFair!();
        RwLockReadGuard!();
        ArcRwLockReadGuard!();
        RwLock!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLockFair , T : ? Sized > ArcRwLockReadGuard < R , T > { # [doc = " Unlocks the `RwLock` using a fair unlock protocol."] # [doc = ""] # [doc = " This is functionally identical to the `unlock_fair` method on [`RwLockReadGuard`]."] # [inline] # [track_caller] pub fn unlock_fair (s : Self) { drop (Self :: into_arc_fair (s)) ; } # [doc = " Unlocks the `RwLock` using a fair unlock protocol and returns the `Arc` that was held by the [`ArcRwLockReadGuard`]."] # [inline] pub fn into_arc_fair (s : Self) -> Arc < RwLock < R , T > > { let s = ManuallyDrop :: new (s) ; unsafe { s . rwlock . raw . unlock_shared_fair () ; ptr :: read (& s . rwlock) } } # [doc = " Temporarily unlocks the `RwLock` to execute the given function."] # [doc = ""] # [doc = " This is functionally identical to the `unlocked_fair` method on [`RwLockReadGuard`]."] # [inline] # [track_caller] pub fn unlocked_fair < F , U > (s : & mut Self , f : F) -> U where F : FnOnce () -> U , { unsafe { s . rwlock . raw . unlock_shared_fair () ; } defer ! (s . rwlock . raw . lock_shared ()) ; f () } # [doc = " Temporarily yields the `RwLock` to a waiting thread if there is one."] # [doc = ""] # [doc = " This is functionally identical to the `bump` method on [`RwLockReadGuard`]."] # [inline] # [track_caller] pub fn bump (s : & mut Self) { unsafe { s . rwlock . raw . bump_shared () ; } } }
    };
}

impl_133!()