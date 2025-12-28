macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_1310 {
    () => {
        deps!();
        impl < T : ? Sized > Deref for MutexGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . mutex . value . get () } } }
    };
}

impl_1310!()