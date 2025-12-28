macro_rules! deps {
    () => {
        OwnedMutexGuard!();
    };
}

macro_rules! impl_1299 {
    () => {
        deps!();
        impl < T : ? Sized > Deref for OwnedMutexGuard < T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . mutex . value . get () } } }
    };
}

impl_1299!()