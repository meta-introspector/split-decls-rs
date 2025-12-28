macro_rules! deps {
    () => {
        Resettable!();
        ValueRange!();
        IntoResettable!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < I : Into < ValueRange > > IntoResettable < ValueRange > for I { fn into_resettable (self) -> Resettable < ValueRange > { Resettable :: Value (self . into ()) } }
    };
}

impl_184!()