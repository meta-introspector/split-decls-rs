macro_rules! deps {
    () => {
        IntoResettable!();
        Resettable!();
        StyledStr!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl IntoResettable < StyledStr > for Option < & 'static str > { fn into_resettable (self) -> Resettable < StyledStr > { match self { Some (s) => Resettable :: Value (s . into ()) , None => Resettable :: Reset , } } }
    };
}

impl_176!();