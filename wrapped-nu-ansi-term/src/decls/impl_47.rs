macro_rules! deps {
    () => {
        AnsiStrings!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'a > fmt :: Display for AnsiStrings < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let f : & mut dyn fmt :: Write = f ; self . write_to_any (f) } }
    };
}

impl_47!()