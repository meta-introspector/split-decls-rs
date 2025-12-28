macro_rules! deps {
    () => {
        RwLockReadGuard!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl < 'a , T > ops :: Deref for RwLockReadGuard < 'a , T > { type Target = T ; fn deref (& self) -> & T { self . data . as_ref () . unwrap () . deref () } }
    };
}

impl_316!()