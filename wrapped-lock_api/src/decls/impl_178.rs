macro_rules! deps {
    () => {
        ArcRwLockUpgradableReadGuard!();
        RawRwLockUpgrade!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        # [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgrade , T : ? Sized > Deref for ArcRwLockUpgradableReadGuard < R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . rwlock . data . get () } } }
    };
}

impl_178!()