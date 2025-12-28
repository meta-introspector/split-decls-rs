macro_rules! deps {
    () => {
        Hash128!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl fmt :: LowerHex for Hash128 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . inner , f) } }
    };
}

impl_11!();