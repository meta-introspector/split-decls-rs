macro_rules! deps {
    () => {
        Markup!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl fmt :: Display for Markup { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . text , f) } }
    };
}

impl_10!();