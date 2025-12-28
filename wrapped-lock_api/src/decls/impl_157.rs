macro_rules! deps {
    () => {
        ArcRwLockWriteGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : ? Sized > Drop for ArcRwLockWriteGuard < R , T > { # [inline] fn drop (& mut self) { unsafe { self . rwlock . raw . unlock_exclusive () ; } } }
    };
}

impl_157!();