macro_rules! deps {
    () => {
        BiLockGuard!();
    };
}

macro_rules! impl_1271 {
    () => {
        deps!();
        impl < T > Deref for BiLockGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . bilock . arc . value . as_ref () . unwrap () . get () } } }
    };
}

impl_1271!()