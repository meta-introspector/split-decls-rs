macro_rules! deps {
    () => {
        ProtocolError!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl fmt :: Display for ProtocolError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_3!()