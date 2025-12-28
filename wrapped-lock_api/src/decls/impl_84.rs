macro_rules! deps {
    () => {
        GetThreadId!();
        RawMutex!();
        ArcReentrantMutexGuard!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawMutex , G : GetThreadId , T : ? Sized > Drop for ArcReentrantMutexGuard < R , G , T > { # [inline] fn drop (& mut self) { unsafe { self . remutex . raw . unlock () ; } } }
    };
}

impl_84!();