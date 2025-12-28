macro_rules! deps {
    () => {
        AngleBracketedArgs!();
        GenericArgs!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl From < AngleBracketedArgs > for Box < GenericArgs > { fn from (val : AngleBracketedArgs) -> Self { Box :: new (GenericArgs :: AngleBracketed (val)) } }
    };
}

impl_23!()