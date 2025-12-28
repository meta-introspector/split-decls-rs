macro_rules! deps {
    () => {
        ValueParser!();
        Resettable!();
        IntoResettable!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < I : Into < ValueParser > > IntoResettable < ValueParser > for I { fn into_resettable (self) -> Resettable < ValueParser > { Resettable :: Value (self . into ()) } }
    };
}

impl_185!();