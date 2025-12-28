macro_rules! deps {
    () => {
        RawMutex!();
        ArcMutexGuard!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] unsafe impl < R : RawMutex + Send + Sync , T : Send + ? Sized > Send for ArcMutexGuard < R , T > where R :: GuardMarker : Send { }
    };
}

impl_32!()