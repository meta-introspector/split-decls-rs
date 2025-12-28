macro_rules! deps {
    () => {
        IntoResettable!();
        Resettable!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl IntoResettable < usize > for usize { fn into_resettable (self) -> Resettable < usize > { Resettable :: Value (self) } }
    };
}

impl_181!();