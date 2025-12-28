macro_rules! deps {
    () => {
        ObjectId!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl std :: fmt :: Display for ObjectId { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . to_hex ()) } }
    };
}

impl_21!();