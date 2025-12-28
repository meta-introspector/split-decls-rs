macro_rules! deps {
    () => {
        ArcReentrantMutexGuard!();
        GetThreadId!();
        RawMutex!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawMutex , G : GetThreadId , T : ? Sized > Deref for ArcReentrantMutexGuard < R , G , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . remutex . data . get () } } }
    };
}

impl_83!()