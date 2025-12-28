macro_rules! deps {
    () => {
        RawRwLock!();
        ArcRwLockReadGuard!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : ? Sized > Deref for ArcRwLockReadGuard < R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . rwlock . data . get () } } }
    };
}

impl_134!()