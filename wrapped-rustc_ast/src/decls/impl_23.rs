macro_rules! deps {
    () => {
        GenericArgs!();
        AngleBracketedArgs!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl From < AngleBracketedArgs > for Box < GenericArgs > { fn from (val : AngleBracketedArgs) -> Self { Box :: new (GenericArgs :: AngleBracketed (val)) } }
    };
}

impl_23!();