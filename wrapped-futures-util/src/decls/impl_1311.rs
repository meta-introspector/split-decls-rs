macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_1311 {
    () => {
        deps!();
        impl < T : ? Sized > DerefMut for MutexGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . mutex . value . get () } } }
    };
}

impl_1311!();