macro_rules! deps {
    () => {
        MaxRecursionReached!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl core :: fmt :: Display for MaxRecursionReached { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str ("Maximum recursion depth has been reached") } }
    };
}

impl_6!()