macro_rules! deps {
    () => {
        ArcMutexGuard!();
        RawMutex!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawMutex , T : ? Sized > Drop for ArcMutexGuard < R , T > { # [inline] fn drop (& mut self) { unsafe { self . mutex . raw . unlock () ; } } }
    };
}

impl_38!()