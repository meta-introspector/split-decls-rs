macro_rules! deps {
    () => {
        Mutex!();
        RawMutex!();
        ArcMutexGuard!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawMutex , T : ? Sized > ArcMutexGuard < R , T > { # [doc = " Returns a reference to the `Mutex` this is guarding, contained in its `Arc`."] # [inline] pub fn mutex (s : & Self) -> & Arc < Mutex < R , T > > { & s . mutex } # [doc = " Unlocks the mutex and returns the `Arc` that was held by the [`ArcMutexGuard`]."] # [inline] # [track_caller] pub fn into_arc (s : Self) -> Arc < Mutex < R , T > > { let s = ManuallyDrop :: new (s) ; unsafe { s . mutex . raw . unlock () ; ptr :: read (& s . mutex) } } # [doc = " Temporarily unlocks the mutex to execute the given function."] # [doc = ""] # [doc = " This is safe because `&mut` guarantees that there exist no other"] # [doc = " references to the data protected by the mutex."] # [inline] # [track_caller] pub fn unlocked < F , U > (s : & mut Self , f : F) -> U where F : FnOnce () -> U , { unsafe { s . mutex . raw . unlock () ; } defer ! (s . mutex . raw . lock ()) ; f () } }
    };
}

impl_34!()