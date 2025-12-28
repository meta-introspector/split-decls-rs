macro_rules! deps {
    () => {
        Resettable!();
        IntoResettable!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < T > IntoResettable < T > for Resettable < T > { fn into_resettable (self) -> Resettable < T > { self } }
    };
}

impl_179!()