macro_rules! deps {
    () => {
        TryLock!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T > Deref for TryLock < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . __ptr . data . get () } } }
    };
}

impl_7!();