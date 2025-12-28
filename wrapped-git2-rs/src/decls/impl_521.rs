macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        impl fmt :: Debug for Oid { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self , f) } }
    };
}

impl_521!();