macro_rules! deps {
    () => {
        IntoResettable!();
        Resettable!();
        OsStr!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < I : Into < OsStr > > IntoResettable < OsStr > for I { fn into_resettable (self) -> Resettable < OsStr > { Resettable :: Value (self . into ()) } }
    };
}

impl_188!();