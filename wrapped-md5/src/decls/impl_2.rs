macro_rules! deps {
    () => {
        Digest!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Digest { # [inline] fn fmt (& self , formatter : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: LowerHex :: fmt (self , formatter) } }
    };
}

impl_2!()