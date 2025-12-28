macro_rules! deps {
    () => {
        MapSpecialCaseFnInto!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < U > Clone for MapSpecialCaseFnInto < U > { # [inline] fn clone (& self) -> Self { Self (PhantomData) } }
    };
}

impl_57!();