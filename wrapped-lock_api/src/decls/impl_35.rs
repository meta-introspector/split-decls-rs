macro_rules! deps {
    () => {
        Mutex!();
        MutexGuard!();
        ArcMutexGuard!();
        RawMutexFair!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawMutexFair , T : ? Sized > ArcMutexGuard < R , T > { # [doc = " Unlocks the mutex using a fair unlock protocol."] # [doc = ""] # [doc = " This is functionally identical to the `unlock_fair` method on [`MutexGuard`]."] # [inline] # [track_caller] pub fn unlock_fair (s : Self) { drop (Self :: into_arc_fair (s)) ; } # [doc = " Unlocks the mutex using a fair unlock protocol and returns the `Arc` that was held by the [`ArcMutexGuard`]."] # [inline] pub fn into_arc_fair (s : Self) -> Arc < Mutex < R , T > > { let s = ManuallyDrop :: new (s) ; unsafe { s . mutex . raw . unlock_fair () ; ptr :: read (& s . mutex) } } # [doc = " Temporarily unlocks the mutex to execute the given function."] # [doc = ""] # [doc = " This is functionally identical to the `unlocked_fair` method on [`MutexGuard`]."] # [inline] # [track_caller] pub fn unlocked_fair < F , U > (s : & mut Self , f : F) -> U where F : FnOnce () -> U , { unsafe { s . mutex . raw . unlock_fair () ; } defer ! (s . mutex . raw . lock ()) ; f () } # [doc = " Temporarily yields the mutex to a waiting thread if there is one."] # [doc = ""] # [doc = " This is functionally identical to the `bump` method on [`MutexGuard`]."] # [inline] # [track_caller] pub fn bump (s : & mut Self) { unsafe { s . mutex . raw . bump () ; } } }
    };
}

impl_35!();