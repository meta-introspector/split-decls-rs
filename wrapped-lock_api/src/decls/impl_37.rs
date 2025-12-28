macro_rules! deps {
    () => {
        RawMutex!();
        ArcMutexGuard!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawMutex , T : ? Sized > DerefMut for ArcMutexGuard < R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . mutex . data . get () } } }
    };
}

impl_37!();