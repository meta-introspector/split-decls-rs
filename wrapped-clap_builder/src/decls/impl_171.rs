macro_rules! deps {
    () => {
        IntoResettable!();
        Resettable!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl IntoResettable < char > for Option < char > { fn into_resettable (self) -> Resettable < char > { match self { Some (s) => Resettable :: Value (s) , None => Resettable :: Reset , } } }
    };
}

impl_171!();