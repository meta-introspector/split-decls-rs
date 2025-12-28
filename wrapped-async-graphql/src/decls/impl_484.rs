macro_rules! deps {
    () => {
        Result!();
        Schema!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl Debug for Schema { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Schema") . finish () } }
    };
}

impl_484!()