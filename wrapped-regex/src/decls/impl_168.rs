macro_rules! deps {
    () => {
        RegexSet!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl core :: fmt :: Debug for RegexSet { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "RegexSet({:?})" , self . patterns ()) } }
    };
}

impl_168!();