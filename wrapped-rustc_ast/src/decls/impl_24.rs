macro_rules! deps {
    () => {
        ParenthesizedArgs!();
        GenericArgs!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl From < ParenthesizedArgs > for Box < GenericArgs > { fn from (val : ParenthesizedArgs) -> Self { Box :: new (GenericArgs :: Parenthesized (val)) } }
    };
}

impl_24!();