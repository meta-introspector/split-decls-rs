macro_rules! deps {
    () => {
        IntoResettable!();
        ArgAction!();
        Resettable!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl IntoResettable < ArgAction > for Option < ArgAction > { fn into_resettable (self) -> Resettable < ArgAction > { match self { Some (s) => Resettable :: Value (s) , None => Resettable :: Reset , } } }
    };
}

impl_173!();