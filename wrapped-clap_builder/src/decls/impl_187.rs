macro_rules! deps {
    () => {
        StyledStr!();
        IntoResettable!();
        Resettable!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < I : Into < StyledStr > > IntoResettable < StyledStr > for I { fn into_resettable (self) -> Resettable < StyledStr > { Resettable :: Value (self . into ()) } }
    };
}

impl_187!();