macro_rules! deps {
    () => {
        DFA!();
        CacheError!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl core :: fmt :: Display for CacheError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "lazy DFA cache has been cleared too many times") } }
    };
}

impl_264!();