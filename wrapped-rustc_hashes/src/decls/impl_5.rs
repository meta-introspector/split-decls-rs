macro_rules! deps {
    () => {
        Hash64!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl fmt :: LowerHex for Hash64 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . inner , f) } }
    };
}

impl_5!()