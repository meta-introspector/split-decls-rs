macro_rules! deps {
    () => {
        AbsPathBuf!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl fmt :: Display for AbsPathBuf { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_13!()