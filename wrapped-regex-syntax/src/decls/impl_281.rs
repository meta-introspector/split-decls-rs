macro_rules! deps {
    () => {
        CaseFoldError!();
        Formatter!();
        Result!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl core :: fmt :: Display for CaseFoldError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Unicode-aware case folding is not available \
             (probably because the unicode-case feature is not enabled)") } }
    };
}

impl_281!();