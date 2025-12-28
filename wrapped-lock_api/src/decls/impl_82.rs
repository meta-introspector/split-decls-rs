macro_rules! deps {
    () => {
        ArcReentrantMutexGuard!();
        ReentrantMutexGuard!();
        GetThreadId!();
        RawMutexFair!();
        ReentrantMutex!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawMutexFair , G : GetThreadId , T : ? Sized > ArcReentrantMutexGuard < R , G , T > { # [doc = " Unlocks the mutex using a fair unlock protocol."] # [doc = ""] # [doc = " This is functionally identical to the `unlock_fair` method on [`ReentrantMutexGuard`]."] # [inline] # [track_caller] pub fn unlock_fair (s : Self) { drop (Self :: into_arc_fair (s)) ; } # [doc = " Unlocks the mutex using a fair unlock protocol and returns the `Arc` that was held by the [`ArcReentrantMutexGuard`]."] # [inline] pub fn into_arc_fair (s : Self) -> Arc < ReentrantMutex < R , G , T > > { let s = ManuallyDrop :: new (s) ; unsafe { s . remutex . raw . unlock_fair () ; ptr :: read (& s . remutex) } } # [doc = " Temporarily unlocks the mutex to execute the given function."] # [doc = ""] # [doc = " This is functionally identical to the `unlocked_fair` method on [`ReentrantMutexGuard`]."] # [inline] # [track_caller] pub fn unlocked_fair < F , U > (s : & mut Self , f : F) -> U where F : FnOnce () -> U , { unsafe { s . remutex . raw . unlock_fair () ; } defer ! (s . remutex . raw . lock ()) ; f () } # [doc = " Temporarily yields the mutex to a waiting thread if there is one."] # [doc = ""] # [doc = " This is functionally equivalent to the `bump` method on [`ReentrantMutexGuard`]."] # [inline] # [track_caller] pub fn bump (s : & mut Self) { unsafe { s . remutex . raw . bump () ; } } }
    };
}

impl_82!();