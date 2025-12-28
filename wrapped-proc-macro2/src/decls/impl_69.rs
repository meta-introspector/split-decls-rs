macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl Debug for Literal { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . inner , f) } }
    };
}

impl_69!()