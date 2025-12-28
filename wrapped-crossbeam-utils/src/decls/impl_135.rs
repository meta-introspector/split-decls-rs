macro_rules! deps {
    () => {
        ShardedLockWriteGuard!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < T : ? Sized > DerefMut for ShardedLockWriteGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . lock . value . get () } } }
    };
}

impl_135!();