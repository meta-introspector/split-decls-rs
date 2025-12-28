macro_rules! deps {
    () => {
        Result!();
        Label!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl fmt :: Display for Label { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_88!();