macro_rules! deps {
    () => {
        Resettable!();
        IntoResettable!();
        Id!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < I : Into < crate :: Id > > IntoResettable < crate :: Id > for I { fn into_resettable (self) -> Resettable < crate :: Id > { Resettable :: Value (self . into ()) } }
    };
}

impl_190!()