macro_rules! deps {
    () => {
        IntoResettable!();
        Resettable!();
        ValueHint!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl IntoResettable < ValueHint > for ValueHint { fn into_resettable (self) -> Resettable < ValueHint > { Resettable :: Value (self) } }
    };
}

impl_183!()