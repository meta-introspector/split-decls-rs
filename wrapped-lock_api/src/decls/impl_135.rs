macro_rules! deps {
    () => {
        RawRwLock!();
        ArcRwLockReadGuard!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : ? Sized > Drop for ArcRwLockReadGuard < R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_shared () ; } } }
    };
}

impl_135!()