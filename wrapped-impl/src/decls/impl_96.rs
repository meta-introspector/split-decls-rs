macro_rules! deps {
    () => {
        Display!();
        IdentUnraw!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl Display for IdentUnraw { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . 0 . unraw () , formatter) } }
    };
}

impl_96!()