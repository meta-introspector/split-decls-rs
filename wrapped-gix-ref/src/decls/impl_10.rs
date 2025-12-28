macro_rules! deps {
    () => {
        FullNameRef!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl std :: fmt :: Display for FullNameRef { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_10!();