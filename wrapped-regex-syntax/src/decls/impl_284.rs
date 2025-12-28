macro_rules! deps {
    () => {
        Result!();
        UnicodeWordError!();
        Formatter!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl core :: fmt :: Display for UnicodeWordError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Unicode-aware \\w class is not available \
             (probably because the unicode-perl feature is not enabled)") } }
    };
}

impl_284!();