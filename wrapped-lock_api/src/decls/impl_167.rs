macro_rules! deps {
    () => {
        RawRwLockUpgrade!();
        RwLockUpgradableReadGuard!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl < 'a , R : RawRwLockUpgrade + 'a , T : ? Sized + 'a > Deref for RwLockUpgradableReadGuard < 'a , R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . rwlock . data . get () } } }
    };
}

impl_167!();