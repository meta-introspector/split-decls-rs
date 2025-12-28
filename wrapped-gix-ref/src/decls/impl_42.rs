macro_rules! deps {
    () => {
        PartialNameRef!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl std :: fmt :: Display for PartialNameRef { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_42!()