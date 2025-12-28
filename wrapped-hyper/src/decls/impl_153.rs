macro_rules! deps {
    () => {
        Result!();
        Protocol!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        # [cfg (feature = "http2")] impl fmt :: Debug for Protocol { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . inner . fmt (f) } }
    };
}

impl_153!()