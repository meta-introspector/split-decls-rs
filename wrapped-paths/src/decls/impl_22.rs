macro_rules! deps {
    () => {
        AbsPath!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl fmt :: Display for AbsPath { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_22!();