macro_rules! deps {
    () => {
        Resettable!();
        Str!();
        IntoResettable!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl IntoResettable < Str > for Option < & 'static str > { fn into_resettable (self) -> Resettable < Str > { match self { Some (s) => Resettable :: Value (s . into ()) , None => Resettable :: Reset , } } }
    };
}

impl_178!()