macro_rules! deps {
    () => {
        CachedStatement!();
        Statement!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'conn > Deref for CachedStatement < 'conn > { type Target = Statement < 'conn > ; # [inline] fn deref (& self) -> & Statement < 'conn > { self . stmt . as_ref () . unwrap () } }
    };
}

impl_64!();