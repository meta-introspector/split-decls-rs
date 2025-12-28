macro_rules! deps {
    () => {
        ArcRwLockWriteGuard!();
        RawRwLock!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : ? Sized > DerefMut for ArcRwLockWriteGuard < R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . rwlock . data . get () } } }
    };
}

impl_156!();