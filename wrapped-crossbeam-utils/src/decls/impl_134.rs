macro_rules! deps {
    () => {
        ShardedLockWriteGuard!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < T : ? Sized > Deref for ShardedLockWriteGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . lock . value . get () } } }
    };
}

impl_134!();