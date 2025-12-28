macro_rules! deps {
    () => {
        IntoResettable!();
        Resettable!();
        ValueParser!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl IntoResettable < ValueParser > for Option < ValueParser > { fn into_resettable (self) -> Resettable < ValueParser > { match self { Some (s) => Resettable :: Value (s) , None => Resettable :: Reset , } } }
    };
}

impl_175!();