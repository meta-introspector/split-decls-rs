macro_rules! deps {
    () => {
        Resettable!();
        IntoResettable!();
        ValueHint!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl IntoResettable < ValueHint > for Option < ValueHint > { fn into_resettable (self) -> Resettable < ValueHint > { match self { Some (s) => Resettable :: Value (s) , None => Resettable :: Reset , } } }
    };
}

impl_174!();