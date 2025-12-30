// Generated macro for impl_101 (impl)
macro_rules! Depcrate_remuteximpl_101 {
() => {
// Module: crate::remutex
// Provides: {"impl_101"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawMutex , G : GetThreadId , T : ? Sized > ArcReentrantMutexGuard < R , G , T > { # [doc = " Returns a reference to the `ReentrantMutex` this object is guarding, contained in its `Arc`."] pub fn remutex (s : & Self) -> & Arc < ReentrantMutex < R , G , T > > { & s . remutex } # [doc = " Unlocks the mutex and returns the `Arc` that was held by the [`ArcReentrantMutexGuard`]."] # [inline] pub fn into_arc (s : Self) -> Arc < ReentrantMutex < R , G , T > > { let s = ManuallyDrop :: new (s) ; unsafe { s . remutex . raw . unlock () ; ptr :: read (& s . remutex) } } # [doc = " Temporarily unlocks the mutex to execute the given function."] # [doc = ""] # [doc = " This is safe because `&mut` guarantees that there exist no other"] # [doc = " references to the data protected by the mutex."] # [inline] # [track_caller] pub fn unlocked < F , U > (s : & mut Self , f : F) -> U where F : FnOnce () -> U , { unsafe { s . remutex . raw . unlock () ; } defer ! (s . remutex . raw . lock ()) ; f () } }
};
}
