macro_rules! deps {
    () => {
        FullName!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl std :: fmt :: Display for FullName { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_9!();