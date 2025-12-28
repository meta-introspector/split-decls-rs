macro_rules! deps {
    () => {
        RequestId!();
        IdRepr!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl fmt :: Display for RequestId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . 0 { IdRepr :: I32 (it) => fmt :: Display :: fmt (it , f) , IdRepr :: String (it) => fmt :: Debug :: fmt (it , f) , } } }
    };
}

impl_18!();