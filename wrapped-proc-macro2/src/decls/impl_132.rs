macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl Display for Literal { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . repr , f) } }
    };
}

impl_132!();