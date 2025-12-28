macro_rules! deps {
    () => {
        ArgAction!();
        Resettable!();
        IntoResettable!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl IntoResettable < ArgAction > for ArgAction { fn into_resettable (self) -> Resettable < ArgAction > { Resettable :: Value (self) } }
    };
}

impl_182!();