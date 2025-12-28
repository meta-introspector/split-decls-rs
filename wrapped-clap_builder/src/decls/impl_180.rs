macro_rules! deps {
    () => {
        IntoResettable!();
        Resettable!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl IntoResettable < char > for char { fn into_resettable (self) -> Resettable < char > { Resettable :: Value (self) } }
    };
}

impl_180!();