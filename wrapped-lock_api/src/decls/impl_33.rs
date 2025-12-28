macro_rules! deps {
    () => {
        RawMutex!();
        ArcMutexGuard!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] unsafe impl < R : RawMutex + Sync , T : Sync + ? Sized > Sync for ArcMutexGuard < R , T > where R :: GuardMarker : Sync { }
    };
}

impl_33!()