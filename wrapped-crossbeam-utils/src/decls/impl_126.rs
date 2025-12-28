macro_rules! deps {
    () => {
        ShardedLockReadGuard!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < T : ? Sized > Deref for ShardedLockReadGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . lock . value . get () } } }
    };
}

impl_126!()