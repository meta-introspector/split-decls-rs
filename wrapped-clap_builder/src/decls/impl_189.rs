macro_rules! deps {
    () => {
        Str!();
        Resettable!();
        IntoResettable!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < I : Into < Str > > IntoResettable < Str > for I { fn into_resettable (self) -> Resettable < Str > { Resettable :: Value (self . into ()) } }
    };
}

impl_189!();