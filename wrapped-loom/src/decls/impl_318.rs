macro_rules! deps {
    () => {
        RwLockWriteGuard!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl < 'a , T > ops :: Deref for RwLockWriteGuard < 'a , T > { type Target = T ; fn deref (& self) -> & T { self . data . as_ref () . unwrap () . deref () } }
    };
}

impl_318!();