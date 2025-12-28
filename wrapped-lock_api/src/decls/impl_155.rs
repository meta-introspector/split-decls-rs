macro_rules! deps {
    () => {
        RawRwLock!();
        ArcRwLockWriteGuard!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : ? Sized > Deref for ArcRwLockWriteGuard < R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . rwlock . data . get () } } }
    };
}

impl_155!()