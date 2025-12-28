macro_rules! deps {
    () => {
        RawMutex!();
        ArcMutexGuard!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawMutex , T : ? Sized > Deref for ArcMutexGuard < R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . mutex . data . get () } } }
    };
}

impl_36!();