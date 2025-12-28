macro_rules! deps {
    () => {
        Result!();
        Value!();
        Map!();
        Formatter!();
        Error!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Debug for Map < String , Value > { # [inline] fn fmt (& self , formatter : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { self . map . fmt (formatter) } }
    };
}

impl_87!()