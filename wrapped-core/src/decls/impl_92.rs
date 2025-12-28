macro_rules! deps {
    () => {
        Digest!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl core :: fmt :: Display for Digest { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { for i in self . data . iter () { write ! (f , "{i:08x}") ? ; } Ok (()) } }
    };
}

impl_92!();