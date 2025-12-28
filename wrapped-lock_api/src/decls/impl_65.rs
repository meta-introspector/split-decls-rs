macro_rules! deps {
    () => {
        ReentrantMutex!();
        RawMutexTimed!();
        GetThreadId!();
        ReentrantMutexGuard!();
        ArcReentrantMutexGuard!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < R : RawMutexTimed , G : GetThreadId , T : ? Sized > ReentrantMutex < R , G , T > { # [doc = " Attempts to acquire this lock until a timeout is reached."] # [doc = ""] # [doc = " If the lock could not be acquired before the timeout expired, then"] # [doc = " `None` is returned. Otherwise, an RAII guard is returned. The lock will"] # [doc = " be unlocked when the guard is dropped."] # [inline] # [track_caller] pub fn try_lock_for (& self , timeout : R :: Duration) -> Option < ReentrantMutexGuard < '_ , R , G , T > > { if self . raw . try_lock_for (timeout) { Some (unsafe { self . make_guard_unchecked () }) } else { None } } # [doc = " Attempts to acquire this lock until a timeout is reached."] # [doc = ""] # [doc = " If the lock could not be acquired before the timeout expired, then"] # [doc = " `None` is returned. Otherwise, an RAII guard is returned. The lock will"] # [doc = " be unlocked when the guard is dropped."] # [inline] # [track_caller] pub fn try_lock_until (& self , timeout : R :: Instant) -> Option < ReentrantMutexGuard < '_ , R , G , T > > { if self . raw . try_lock_until (timeout) { Some (unsafe { self . make_guard_unchecked () }) } else { None } } # [doc = " Attempts to acquire this lock until a timeout is reached, through an `Arc`."] # [doc = ""] # [doc = " This method is similar to the `try_lock_for` method; however, it requires the `ReentrantMutex` to be"] # [doc = " inside of an `Arc` and the resulting mutex guard has no lifetime requirements."] # [cfg (feature = "arc_lock")] # [inline] # [track_caller] pub fn try_lock_arc_for (self : & Arc < Self > , timeout : R :: Duration ,) -> Option < ArcReentrantMutexGuard < R , G , T > > { if self . raw . try_lock_for (timeout) { Some (unsafe { self . make_arc_guard_unchecked () }) } else { None } } # [doc = " Attempts to acquire this lock until a timeout is reached, through an `Arc`."] # [doc = ""] # [doc = " This method is similar to the `try_lock_until` method; however, it requires the `ReentrantMutex` to be"] # [doc = " inside of an `Arc` and the resulting mutex guard has no lifetime requirements."] # [cfg (feature = "arc_lock")] # [inline] # [track_caller] pub fn try_lock_arc_until (self : & Arc < Self > , timeout : R :: Instant ,) -> Option < ArcReentrantMutexGuard < R , G , T > > { if self . raw . try_lock_until (timeout) { Some (unsafe { self . make_arc_guard_unchecked () }) } else { None } } }
    };
}

impl_65!();