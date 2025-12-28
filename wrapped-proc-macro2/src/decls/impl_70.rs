macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl Display for Literal { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . inner , f) } }
    };
}

impl_70!()