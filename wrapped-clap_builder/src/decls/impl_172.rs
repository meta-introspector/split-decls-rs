macro_rules! deps {
    () => {
        IntoResettable!();
        Resettable!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl IntoResettable < usize > for Option < usize > { fn into_resettable (self) -> Resettable < usize > { match self { Some (s) => Resettable :: Value (s) , None => Resettable :: Reset , } } }
    };
}

impl_172!();