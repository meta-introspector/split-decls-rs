macro_rules! deps {
    () => {
        Formatter!();
        Value!();
        Map!();
        Error!();
        Result!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Debug for Map < String , Value > { # [inline] fn fmt (& self , formatter : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { self . map . fmt (formatter) } }
    };
}

impl_87!();