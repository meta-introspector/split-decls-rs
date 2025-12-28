macro_rules! deps {
    () => {
        AnsiString!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'a > fmt :: Display for AnsiString < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let w : & mut dyn fmt :: Write = f ; self . write_to_any (w) } }
    };
}

impl_44!();