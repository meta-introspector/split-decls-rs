macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl core :: fmt :: Display for Empty { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Tried to create a `rand::distr::slice::Choose` with an empty slice") } }
    };
}

impl_121!();