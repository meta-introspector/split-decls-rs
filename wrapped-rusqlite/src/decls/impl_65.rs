macro_rules! deps {
    () => {
        Statement!();
        CachedStatement!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < 'conn > DerefMut for CachedStatement < 'conn > { # [inline] fn deref_mut (& mut self) -> & mut Statement < 'conn > { self . stmt . as_mut () . unwrap () } }
    };
}

impl_65!();