macro_rules! deps {
    () => {
        OsStr!();
        IntoResettable!();
        Resettable!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl IntoResettable < OsStr > for Option < & 'static str > { fn into_resettable (self) -> Resettable < OsStr > { match self { Some (s) => Resettable :: Value (s . into ()) , None => Resettable :: Reset , } } }
    };
}

impl_177!()