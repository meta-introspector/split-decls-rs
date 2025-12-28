macro_rules! deps {
    () => {
        MutexGuard!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > ops :: Deref for MutexGuard < 'a , T > { type Target = T ; fn deref (& self) -> & T { self . data . as_ref () . unwrap () . deref () } }
    };
}

impl_302!()