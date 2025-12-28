macro_rules! deps {
    () => {
        Resettable!();
        IntoResettable!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < I : Into < String > > IntoResettable < String > for I { fn into_resettable (self) -> Resettable < String > { Resettable :: Value (self . into ()) } }
    };
}

impl_186!()