macro_rules! deps {
    () => {
        Statement!();
        CachedStatement!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'conn > Deref for CachedStatement < 'conn > { type Target = Statement < 'conn > ; # [inline] fn deref (& self) -> & Statement < 'conn > { self . stmt . as_ref () . unwrap () } }
    };
}

impl_64!()