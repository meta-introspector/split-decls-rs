macro_rules! deps {
    () => {
        Result!();
        MaxRecursionReached!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl core :: fmt :: Display for MaxRecursionReached { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str ("Maximum recursion depth has been reached") } }
    };
}

impl_170!();