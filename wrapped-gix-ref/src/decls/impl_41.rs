macro_rules! deps {
    () => {
        PartialName!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl std :: fmt :: Display for PartialName { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_41!();