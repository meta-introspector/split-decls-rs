macro_rules! deps {
    () => {
        OsStr!();
        IntoResettable!();
        Resettable!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < I : Into < OsStr > > IntoResettable < OsStr > for I { fn into_resettable (self) -> Resettable < OsStr > { Resettable :: Value (self . into ()) } }
    };
}

impl_188!()