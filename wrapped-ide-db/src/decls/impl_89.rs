macro_rules! deps {
    () => {
        Label!();
        Result!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl fmt :: Debug for Label { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& self . 0 , f) } }
    };
}

impl_89!();