macro_rules! deps {
    () => {
        PrivateKey!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl core :: fmt :: Debug for PrivateKey { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "{} {{***OMITTED***}}" , stringify ! (PrivateKey)) } }
    };
}

impl_387!();