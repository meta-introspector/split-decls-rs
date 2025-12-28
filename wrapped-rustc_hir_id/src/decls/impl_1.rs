macro_rules! deps {
    () => {
        OwnerId!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl Debug for OwnerId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { Debug :: fmt (& self . def_id , f) } }
    };
}

impl_1!()